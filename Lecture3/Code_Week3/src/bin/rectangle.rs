#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn touches(&self, other: &Rectangle) -> bool {
        self.x <= other.x + other.width
            && other.x <= self.x + self.width
            && self.y <= other.y + other.height
            && other.y <= self.y + self.height
    }
}

fn main() {
    let a = Rectangle::new(0.0, 0.0, 2.0, 2.0);
    let b = Rectangle::new(1.0, 1.0, 2.0, 2.0);   // overlaps a
    let c = Rectangle::new(2.0, 0.0, 1.0, 2.0);   // edge-touches a
    let d = Rectangle::new(5.0, 5.0, 1.0, 1.0);   // disjoint from a

    println!("a = {:?}", a);
    println!("a.touches(&b) = {}  (overlap)",      a.touches(&b));
    println!("a.touches(&c) = {}  (edge contact)", a.touches(&c));
    println!("a.touches(&d) = {}  (disjoint)",     a.touches(&d));
}
