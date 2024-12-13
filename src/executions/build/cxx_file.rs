use std::{path::{self, Path, PathBuf}, process, str::FromStr};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

use crate::error::EmbargoError;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CxxFileType {
    Source,
    Header,
}

fn modified_default() -> bool {
    false
}

fn serialize_modified<S>(modified: &u64, s: S) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    let mod_str = format!("{:X}", modified);
    s.serialize_str(&mod_str)
}

fn deserialize_modified<'de, D>(d: D) -> Result<u64, D::Error>
where 
    D: Deserializer<'de>
{
    struct ModifiedVisitor;

    impl<'de> Visitor<'de> for ModifiedVisitor {
        type Value = u64;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a hex value that maps to a u64")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where 
            E: de::Error {
                return match u64::from_str_radix(value, 16) {
                    Ok(n) => Ok(n),
                    Err(_) => Err(E::custom(format!("Input must be a hex string!")))
                }
                
            }
    }
    d.deserialize_any(ModifiedVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CxxFile {
    pub file_type: CxxFileType,

    // flag indicating file has been changed
    #[serde(skip, default = "modified_default")]
    changed: bool, 

    #[serde(serialize_with = "serialize_modified", deserialize_with = "deserialize_modified")]
    modified: u64,

    dependencies: Vec<PathBuf>,

    pub dependees: Vec<PathBuf>,
}

impl CxxFile {

    pub fn new(file_type: CxxFileType, modified: u64, path: &Path) -> Result<Self, EmbargoError> {
        let changed = false;
        let dependencies = match gen_deps(path) {
            Ok(dep) => dep,
            Err(e) => {
                return Err(EmbargoError::new(&e))
            }
        };
        let dependees = Vec::new();

        Ok(Self {
            file_type,
            changed,
            modified,
            dependencies,
            dependees
        })
    }

    pub fn modified(&self) -> u64 {
        self.modified
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn set_changed(&mut self) {
        self.changed = true;
    }

    pub fn dependencies(&self) -> &[PathBuf] {
        self.dependencies.as_slice()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectFile {
    #[serde(serialize_with = "serialize_modified", deserialize_with = "deserialize_modified")]
    modified: u64,
}

impl ObjectFile {

    pub fn new(modified: u64) -> Self {
        Self {
            modified
        }
    }

}

fn gen_deps(path: &Path) -> Result<Vec<PathBuf>, String> {

    // TODO: get compiler

    let output = process::Command::new("g++")
        .arg(path.to_str().unwrap_or_default())
        .args(["-MM", "-MG"]).output();

    return match output {
        Ok(output) => {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                let o = s.split_whitespace()
                    .enumerate()
                    .filter(|(i, s)| (*i > 2) && (*s != "\\")) // skip first 2 entries
                    .map(|(_, p)| PathBuf::from_str(p))
                    .filter_map(|p| p.ok())
                    .collect();

                    Ok(o)
            } else {
                let s = String::from_utf8_lossy(&output.stderr);
                Err(s.to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}