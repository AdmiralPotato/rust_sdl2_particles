#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, v: Point) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

pub struct Particle {
    pub pos: Point,
    pub vel: Point,
    pub lifespan: u32,
    pub index: u32,
}

impl Particle {
    pub fn new(x: f32, y: f32, index: u32) -> Particle {
        Particle {
            pos: Point { x, y },
            vel: Point { x: 1.0, y: 0.0 },
            lifespan: 160,
            index,
        }
    }
    pub fn tick(&mut self) {
        self.lifespan -= 1;
        self.pos = self.pos + self.vel;
    }
}
