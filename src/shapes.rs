// pub is required to access this elsewhere
// public & private are the only two accesiblities (no protected is needed)
// values of a struct are also private by default, even if the struct is public
// functions should be used to set these instead within a impl block

/// Rectangle
pub struct Rect {
    // doc strings can be used with ///
    width: u32,
    height: u32,
}

// Circle
pub struct Circle {
    radius: f64,
}

// implementation of method for struct
// structs are now cool
impl Rect {
    /// Creates equal sided rectangle
    pub fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }

    /// Creates rectangle using a width & height
    pub fn rect(width: u32, height: u32) -> Rect {
        Rect { width: width, height: height }
    }

    /// Returns the perimiter of a rectangle
    pub fn perimiter(&self) -> u32 {
        2*(self.width + self.height)
    }

    /// Returns the area of a rect
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Circle {
    /// Creates a new circle with a given radius
    pub fn new(radius: f64) -> Circle {
        Circle { radius: radius }
    }

    /// Returns the circumference of a circle
    pub fn circumference(&self) -> f64 {
        2f64 * 3.14159 * self.radius
    }

    /// Returns the area of a circle
    pub fn area(&self) -> f64 {
        3.14159 * self.radius.powf(2f64)
    }
}
