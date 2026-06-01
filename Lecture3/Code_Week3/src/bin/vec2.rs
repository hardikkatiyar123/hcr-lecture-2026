#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
}

fn main() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);

    println!("a            = {:?}", a);
    println!("b            = {:?}", b);
    println!("a.dot(b)     = {}", a.dot(b));
    println!("a.length()   = {}", a.length());
}
