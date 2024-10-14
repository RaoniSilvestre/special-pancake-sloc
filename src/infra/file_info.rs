use super::FileInfo;
use std::ops::{Add, AddAssign};

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

impl FileInfo {
    pub fn new(code: u32, whitespace: u32, comment: u32) -> FileInfo {
        FileInfo {
            code,
            whitespace,
            comment,
        }
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.code += rhs.code;
        self.whitespace += rhs.whitespace;
        self.comment += rhs.comment;
    }
}
