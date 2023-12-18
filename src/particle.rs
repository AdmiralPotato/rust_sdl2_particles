use nalgebra::{Point2, Vector2};
pub type Point = Point2<f32>;
pub type Vector = Vector2<f32>;

/*
Note for future Admiral: If you find yourself needing to turn a Point into a
Vector, (for instance, when averaging points) use my_point.coords instead of
my_point.into(). my_point.coords will just give you the coordinates,
as a vector. .into() will convert it into a *homogenous* vector, `x, y, 1`.
*/

pub struct Particle {
    pub pos: Point,
    pub vel: Vector,
    pub lifespan: u32,
    pub index: u32,
}

impl Particle {
    pub fn new(x: f32, y: f32, index: u32) -> Particle {
        Particle {
            pos: Point::new(x, y),
            vel: Vector::new(1.0, 0.0),
            lifespan: 160,
            index,
        }
    }
    pub fn tick(&mut self) {
        self.lifespan -= 1;
        self.pos = self.pos + self.vel;
    }
}
