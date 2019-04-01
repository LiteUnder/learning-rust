pub struct Rect {
    width: u32,
    height: u32
}

// implementation of method for struct
// structs are now cool
impl Rect {
    pub fn square(size: u32) -> Rect {
        Rect { width: size, height: size}
    }

    pub fn rect(width: u32, height: u32) -> Rect {
        Rect { width: width, height: height}
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

