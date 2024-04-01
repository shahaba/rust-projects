pub struct Rectangle {
    pub width: u32,
    pub length: u32,
}

impl Rectangle {
    pub fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width >= rect2.width && self.length >= rect2.length
    }
}
