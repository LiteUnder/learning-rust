pub struct Rect {
    width: u32,
    height: u32,
}

pub fn buildRect(width: u32, height: u32) -> Rect {
    Rect {
        width: width,
        height: height,
    }
}

pub fn area(rectangle: Rect) -> u32 {
    rectangle.width * rectangle.height
}