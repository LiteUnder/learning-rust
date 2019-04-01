pub struct Rect {
    width: u32,
    height: u32,
}

pub struct Circle {
    radius: f64,
}

// implementation of method for struct
// structs are now cool
impl Rect {
    pub fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }

    pub fn rect(width: u32, height: u32) -> Rect {
        Rect { width: width, height: height }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius: radius }
    }

    pub fn circumference(&self) -> f64 {
        2f64 * 3.14159 * self.radius
    }

    pub fn area(&self) -> f64 {
        3.14159 * self.radius.powf(2f64)
    }
}
