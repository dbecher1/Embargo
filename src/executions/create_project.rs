use crate::{
    commands::NewArgs,
    embargo_toml::EmbargoFile,
    error::{
        EmbargoError,
        EmbargoResult
    }
};
use std::{
    env,
    fs::{
        self,
        File
    },
    io::Write,
    path::PathBuf,
    process::Command,
};
use log::debug;

pub fn create_project(args: Option<NewArgs>) -> EmbargoResult {
    // newargs: if exists, is the result of embargo new ____
    // if doesn't exist, embargo init

    let mut cwd = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            
            return Err(EmbargoError::new("could not create new directory."));
        },
    };

    let (package_name, package_dir, is_init_cmd) = if let Some(args) = args {
        let name = args.name();

        let mut src_dir = PathBuf::from(name);
        src_dir.push("src");

        if let Err(_) = fs::create_dir_all(src_dir) {
            return Err(EmbargoError::new("a directory by this name may already exist."));
        }

        cwd.push(name);
        (name.to_owned(), Some(name.to_owned()), false)
    } else {
        if let Err(_) = fs::create_dir("src") {
            debug!("Directory src already exists. May want to flag this for an error in the future");
            // return Err(EmbargoError::new("could not create new directory."));
        }
        (
            cwd.file_name().unwrap_or_default().to_str().unwrap_or_default().to_owned(),
            None,
            true,
        )
    };

    let gitignore_contents = include_str!("../../template_files/.gitignore");
    let main_cpp_contents = include_str!("../../template_files/main.cpp");

    let mut config_file = EmbargoFile::default();
    config_file.package.name = package_name;
    let config_contents = match toml::to_string_pretty(&config_file) {
        Ok(toml) => toml,
        Err(_) => {
            return Err(EmbargoError::new("error creating Embargo.toml."));
        }
    };

    let (
        gitignore_path,
        main_cpp_path,
        config_path
    ) = if let Some(directory) = package_dir {
        let (mut temp1, mut temp2, mut temp3) = (
            directory.clone(),
            directory.clone(),
            directory.clone(),
        );
        temp1.push_str("/.gitignore");
        temp2.push_str("/src/main.cpp");
        temp3.push_str("/Embargo.toml");

        (
            temp1,
            temp2,
            temp3
        )
    } else {
        (
            ".gitignore".to_owned(),
            "src/main.cpp".to_owned(),
            "./Embargo.toml".to_owned(),
        )
    };

    debug!("{:?}", config_path);

    let (
        gitignore,
        main_cpp,
        config,
    ) = (
        File::create(gitignore_path),
        File::create(main_cpp_path),
        File::create(config_path),
    );

    let (
        mut gitignore,
        mut main_cpp,
        mut config,
    ) = match (gitignore, main_cpp, config) {

        (Err(_), _, _) => {
            return Err(EmbargoError::new("failed to create file: .gitignore"));
        },

        (_, Err(_), _) => {
            return Err(EmbargoError::new("failed to create file: main.cpp"));
        },

        (_, _, Err(_)) => {
            return Err(EmbargoError::new("failed to create file: Embargo.toml"));
        },

        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
    };

    match (
        gitignore.write(gitignore_contents.as_bytes()),
        main_cpp.write(main_cpp_contents.as_bytes()),
        config.write(config_contents.as_bytes()),
    ) {
        (Err(_), _, _) => {
            return Err(EmbargoError::new("failed to write file: .gitignore"));
        },
        
        (_, Err(_), _) => {
            return Err(EmbargoError::new("failed to write file: main.cpp"));
        },

        (_, _, Err(_)) => {
            return Err(EmbargoError::new("failed to write file: Embargo.toml"));
        },

        _ => {},
    }

    match Command::new("git").arg("init").output() {
        Ok(out) => {
            if !out.status.success() {
                return Err(EmbargoError::new(&String::from_utf8_lossy(&out.stderr)))
            }
        },
        Err(_) => return Err(EmbargoError::new("could not initialize git repository; ensure git is installed and try again.")),
    }

    return if is_init_cmd {
        Ok("Successfully initialized new project".to_owned())
    } else {
        Ok("Successfully created new project".to_owned())
    }
}