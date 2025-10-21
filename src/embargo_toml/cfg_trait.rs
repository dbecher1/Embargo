#[allow(unused)]
pub trait ConfigFile<'a> {
    fn compiler(&'a self) -> &'a str;
    fn linker(&'a self) -> &'a str;
    fn source_path(&'a self) -> &'a str;
    fn build_path(&'a self) -> &'a str;
    fn auto_clean(&'a self) -> bool;
    fn object_path(&'a self) -> &'a str;
    fn target_path_debug(&'a self) -> &'a str;
    fn target_path_release(&'a self) -> &'a str;
    fn bin_path(&'a self) -> &'a str;
    fn lib_path(&'a self) -> &'a str;
    fn flags(&'a self) -> &'a [String];
    fn args(&'a self) -> &'a [String];
    fn author(&'a self) -> Option<&'a str>;
}
