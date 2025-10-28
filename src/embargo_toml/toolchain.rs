use serde::Deserialize;


#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Toolchain {
    Llvm,
    Gcc,
}

impl Toolchain {

    // TODO: once C is implemented, check for that
    pub fn compiler(&self) -> &str {
        // check for
        match self {
            Toolchain::Llvm => {
                "clang++"
            },
            Toolchain::Gcc => "g++",
        }
    }

    pub fn linker(&self) -> &str {
        match self {
            Toolchain::Llvm => {
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
            Toolchain::Gcc => "g++",
        }
    }
}

impl Default for Toolchain {
    fn default() -> Self {
        Self::Gcc
    }
}