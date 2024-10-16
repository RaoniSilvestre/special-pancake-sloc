pub mod file_info;

#[derive(Debug, Clone, Copy, Default)]
pub struct FileInfo {
    pub code: u32,
    pub whitespace: u32,
    pub comment: u32,
}
