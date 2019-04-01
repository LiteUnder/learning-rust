pub struct Rect {
    width: u32,
    height: u32,
}

// I wonder if this could be implemented
pub fn build_rect(width: u32, height: u32) -> Rect {
    Rect {
        width: width,
        height: height,
    }
}

// implementation of method for struct
// structs are now cool
impl Rect {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}