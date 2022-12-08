use crate::rays::Ray;

// Eventually this will be for general shape functions
// or for processing lists of shapes

/*use crate::points::point_subtract;
//use crate::points::to_vector;
use crate::points::Point;
use crate::rays::Ray;
use crate::vectors::dot_product;
use crate::vectors::Vector;*/

//------------------------------------------------------------------------
//General Shape structures and functions
//------------------------------------------------------------------------

pub struct Hue {
    r: f32,
    g: f32,
    b: f32,
}

pub trait Shape {
    fn intersects(&self, ray: Ray);
    fn intersection_point(&self, ray: Ray);
}

//pub trait ShapeAttributes {}

/*struct Shape<D: ShapeAttributes> {
    color: Hue,
}
*/
