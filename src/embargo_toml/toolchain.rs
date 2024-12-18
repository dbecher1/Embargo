use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Toolchain {
    LLVM,
    GCC,
}

impl Toolchain {

    // TODO: once C is implemented, check for that
    pub fn compiler(&self) -> &str {
        // check for
        match self {
            Toolchain::LLVM => {
                "clang++"
            },
            Toolchain::GCC => "g++",
        }
    }

    pub fn linker(&self) -> &str {
        match self {
            Toolchain::LLVM => {
                if cfg!(target_os = "macos") {
                    "lld64.lld"
                } else if cfg!(unix) {
                    "ld.lld"
                } else if cfg!(windows) {
                    "lld-link"
                } else {
                    "ld"
                }
            },
            Toolchain::GCC => "g++",
        }
    }
}