#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset(isize, isize);

impl Offset {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}
