use crate::points;
use crate::vectors;

//------------------------------------------------------------------------
//Spheres
//------------------------------------------------------------------------

struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn on_surface(&self, point: Point) -> bool {
        (self.center.x - point.x).pow(2)
            + (self.center.y - point.y).pow(2)
            + (self.center.z - point.y).pow(2)
            == self.radius
    }
}
//maybe make is_intersection for each shape as impl function
pub fn is_intersection(sphere: Sphere, ray: Ray, t: f32) -> bool {}
