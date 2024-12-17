use std::{
    cell::RefCell,
    ffi::OsStr,
    fs::{self, File},
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::Instant
};
use build_file::EmbargoBuildFile;
use cxx_file::{CxxFile, CxxFileType};
use log::debug;
use topological_sort::TopologicalSort;
use walkdir::{DirEntry, WalkDir};
use crate::{
    commands::BuildArgs,
    embargo_toml::{EmbargoFile, GlobalEmbargoFile},
    error::{EmbargoError, EmbargoResult}
};

mod cxx_file;
mod build_file;
mod serde_helpers;

#[allow(unused_variables)]
pub fn build_project(args: BuildArgs, global_file: &GlobalEmbargoFile, embargo_toml: &EmbargoFile, embargo_toml_path: &Path) -> EmbargoResult {

    let now = Instant::now();
    
    let mut cwd = embargo_toml_path.to_path_buf();

    // pop the toml file from the path
    cwd.pop();

    println!("Compiling {} v{} ({})", embargo_toml.package.name, embargo_toml.package.version, cwd.display());

    // Check to see if there are overridden values in the Embargo.toml file
    let mut src_dir = cwd.clone();

    if let Some(src_dir_override) = &embargo_toml.package.source_path {
        src_dir.push(src_dir_override);
    } else {
        src_dir.push(&global_file.source_path);
    }

    let mut buildfile_path = cwd.clone();

    if let Some(build_path_override) = &embargo_toml.package.build_path {
        buildfile_path.push(build_path_override);
    } else {
        buildfile_path.push(&global_file.build_path);
    }

    // Create the build file path if it doesn't exist
    // Don't care about the result 
    let _ = fs::create_dir_all(&buildfile_path);

    // Now try to read the build file if it exists
    buildfile_path.push("Embargo.build");

    let embargo_build = match fs::read_to_string(&buildfile_path) {
        Ok(file) => {
            let toml: EmbargoBuildFile = toml::from_str(&file)?;
            Some(toml)
        },

        // If there's an error, this may be a new build
        // So gracefully keep going
        Err(_) => None, 
    };

    // the file that will be written to at the end of the build
    // This represents the current state of the project
    // Whereas the file we read from above would represent the previous build
    let mut new_embargo_build = EmbargoBuildFile::new();

    // We want to recompile if Embargo.toml was modified, even if the source files werent
    let embargo_toml_modified = if let Some(ref embargo_build) = embargo_build {
        let h = hash_helper(embargo_toml);
        // set the new file while we have this value
        new_embargo_build.embargo_toml_modified = h;

        if h != embargo_build.embargo_toml_modified {
            true
        } else {
            false
        }
    } else {
        false
    };
    
    
    for entry in WalkDir::new(src_dir.clone())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_valid_file_ext(e.path())) {
            
            let mod_time = entry.metadata()?.modified()?;

            let path = entry.path();
            let filename = path.file_name().unwrap_or_default();

            debug!("Reading {}", filename.to_str().unwrap_or_default());

            let ext = path.extension().unwrap_or_default();
            
            let hash = hash_helper(mod_time);

            let file_type = if is_source(ext) {
                CxxFileType::Source
            } else if is_header(ext) {
                CxxFileType::Header
            } else {
                continue;
            };

            // The dependency list is generated within the below constructor
            // I might want to move it to right before compilation, after re-compile checks are done - I'll experiment with it at a future time
            let new_cxx_file = CxxFile::new(file_type, hash, path)?;

            new_embargo_build.source_files.insert(path.to_path_buf(), RefCell::new(new_cxx_file));
        }

        // The bool flag is set if this is a completely fresh build
        let (fresh_build, files_changed) = if let Some(old_build) = embargo_build {

            let mut files_changed = false; // flag that's set once a change has been detected - prevents rebuilds if nothing has changed

            for (path, file) in new_embargo_build.source_files.iter() {
                // This is the comparison to see if this file was modified
                // If this is not some, this is a new file in the build
                // If this is some, check to see if the file was modified
                if let Some(last_file) = old_build.source_files.get(path) {
                    if file.borrow().modified() != last_file.borrow().modified() {
                        if !files_changed {
                            files_changed = true;
                        }

                        file.borrow_mut().set_changed();
                    }
                }
            }

            // check dependencies once we've done the fist pass 
            if files_changed {

                let mut ts = TopologicalSort::<PathBuf>::new();
                
                for (path, file) in new_embargo_build.source_files.iter() {
                    for dep in file.borrow().dependencies() {
                        ts.add_dependency(dep, path);
                    }
                }
                
                while let Some(path) = ts.pop() {
                    // look up the file
                    if let Some(file) = new_embargo_build.source_files.get(&path) {

                        // skip if already marked changed
                        if file.borrow().changed() {
                            continue;
                        }
                        let deps = file.borrow().dependencies().to_vec();
                        for dep_path in &deps {
                            // look up the dependency files and check if those are changed
                            if let Some(dep_file) = new_embargo_build.source_files.get(dep_path) {
                                if dep_file.borrow().changed() {
                                    file.borrow_mut().set_changed();
                                }
                            }
                        }
                    }
                }
                
            }

            (false, files_changed)
        } else {
            // files changed is always true in a new project
            (true, true)
        };

        if cfg!(debug_assertions) {
            for (path, file) in &new_embargo_build.source_files {
                if file.borrow().changed() {
                    debug!("File changed: {}", path.display());
                }
            }
        }

        // Create the object and binary path path
        let mut object_path = buildfile_path.clone();
        object_path.pop();

        // set overrides if they exist
        if let Some(op) = &embargo_toml.package.object_path {
            object_path.push(op);
        } else {
            object_path.push(&global_file.object_path);
        }
        let _ = fs::create_dir_all(&object_path);

        let mut bin_path = buildfile_path.clone();
        bin_path.pop();

        if let Some(bp) = &embargo_toml.package.bin_path {
            bin_path.push(bp);
        } else {
            bin_path.push(&global_file.bin_path);
        }
        let _ = fs::create_dir_all(&bin_path);
        bin_path.push(&embargo_toml.package.name);

        // COMPILE
        
        for (path, file) in new_embargo_build.source_files
            .iter()
            .filter(|(p, f)| (
                    f.borrow().changed() ||
                    fresh_build ||
                    embargo_toml_modified
                ) && is_source(p.extension().unwrap_or_default()))
            {
                let mut object_path = object_path.clone();
                
                let mut command = Command::new(global_file.cxx_compiler());

                let mut args = Vec::new();
                args.push("-c");
                args.push(path.as_os_str().to_str().unwrap_or_default());

                args.push("-o");

                let filename = path.file_name().unwrap_or_default();
                let filename = filename.to_str().unwrap_or_default();
                let filename_o = filename.replace("cpp", "o");
                object_path.push(filename_o);
                
                args.push(object_path.to_str().unwrap_or_default());

                debug!("{} {}", global_file.cxx_compiler(), args.iter().fold(String::new(), |s, a| s + " " + a));

                match command.args(args).output() {
                    Ok(output) => {
                        if output.status.success() {
                            debug!("Compiling {}...", filename);
                        } else {
                            return Err(EmbargoError::new(&String::from_utf8_lossy(&output.stderr)));
                        }
                    },
                    Err(_) => {
                        return Err(EmbargoError::new("compilation failed"));
                    }
                }
            }
        
        if !files_changed && !embargo_toml_modified {
            return Ok(Some("No changed files detected.".to_owned()))
        }

        debug!("Linking object files...");

        let objects = WalkDir::new(object_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_obj_file(e))
            .map(|e| e.path().as_os_str().to_str().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();

        // linker args
        let mut args = Vec::new();
        args.push("-o".to_owned());
        args.push(bin_path.as_os_str().to_str().unwrap_or_default().to_owned());

        for o in objects {
            args.push(o);
        }

        match Command::new(global_file.cxx_compiler()).args(args).output() {
            Ok(output) => {
                if !output.status.success() {
                    return Err(EmbargoError::new(&String::from_utf8_lossy(&output.stderr)));
                } 
            },
            Err(_) => {
                return Err(EmbargoError::new("error linking executable"))
            }
        }
       
        // Save the new file!

        let new_buildfile = match File::create(buildfile_path.clone()) {
            Ok(bf) => Some(bf),
            Err(_) => None,
        };
        
        let new_str = toml::to_string_pretty(&new_embargo_build)?;
        if let Some(mut file) = new_buildfile {
            file.write(new_str.as_bytes())?;
        }

    let build_time = (now.elapsed().as_millis() as f32) / 1000.;

    println!("Finished compiling project in {build_time:.2}s");

    Ok(None)
}

fn hash_helper<T: Hash>(t: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// Helper things

fn is_source(entry: &OsStr) -> bool {
    entry == "cpp" || entry == "c"
}

fn is_header(ext: &OsStr) -> bool {
    ext == "h" ||
    ext == "hpp"
}

fn is_obj_file(entry: &DirEntry) -> bool {
    let p = entry.path();
    let ext = p.extension().unwrap_or_default();
    ext == "o"
}

fn is_valid_file_ext(path: &Path) -> bool {
    let ext = path.extension().unwrap_or_default();
    is_header(ext) || is_source(ext)
}