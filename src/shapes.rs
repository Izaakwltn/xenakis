use crate::points::Point;
use crate::rays::Ray;

//------------------------------------------------------------------------
//General Shape structures and functions
//------------------------------------------------------------------------

#[derive(Debug)]
pub struct Hue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Hue {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

pub trait Shape {
    fn intersects(&self, ray: Ray) -> bool;
    fn intersection_point(&self, ray: Ray) -> Point;
}

//pub trait ShapeAttributes {}

/*struct Shape<D: ShapeAttributes> {
    color: Hue,
}
*/
