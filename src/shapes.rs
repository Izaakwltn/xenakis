use crate::points::Point;
use crate::rays::Ray;

//------------------------------------------------------------------------
//General Shape structures and functions
//------------------------------------------------------------------------

#[derive(Debug)]
pub struct Hue {
    r: f32,
    g: f32,
    b: f32,
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
