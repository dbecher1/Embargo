
#[allow(unused)]
pub trait ConfigFile<'a> {
    fn compiler(&'a self) -> &'a str;
    fn linker(&'a self) -> &'a str;
    fn source_path(&'a self) -> &'a str;
    fn build_path(&'a self) -> String;
    fn auto_clean(&'a self) -> bool;
    fn object_path(&'a self) -> String;
    fn target_path_debug(&'a self) -> String;
    fn target_path_release(&'a self) -> String;
    fn bin_path(&'a self) -> String;
    fn lib_path(&'a self) -> String;
    fn flags(&'a self) -> Vec<String>;
    fn args(&'a self) -> Vec<String>;
    fn author(&'a self) -> Option<&'a str>;
}