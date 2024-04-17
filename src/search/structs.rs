use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub code: u32,
    pub whitespace: u32,
    pub comment: u32,
}

impl Add for FileInfo {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        FileInfo {
            code: self.code + other.code,
            whitespace: self.whitespace + other.whitespace,
            comment: self.comment + other.comment,
        }
    }
}
