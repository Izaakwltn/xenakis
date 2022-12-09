use crate::points::Point;
use crate::vectors::Vector;

//------------------------------------------------------------------------
//Rays
//------------------------------------------------------------------------

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub t_max: f32,
}

//Making new Rays
impl Ray {
    pub fn new(origin: Point, direction: Vector, t_max: f32) -> Self {
        Self {
            origin,
            direction,
            t_max,
        }
    }
    pub fn default() -> Self {
        Self::new(Point::default(), Vector::default(), 1000000000000000.0)
    }
}

impl Copy for Ray {}

impl Clone for Ray {
    fn clone(&self) -> Ray {
        *self
    }
}

impl Ray {
    pub fn normalize_direction(&self) -> Vector {
        let magn = self.direction.length();
        Vector::new(
            self.direction.x / magn,
            self.direction.y / magn,
            self.direction.z / magn,
        )
    }

    pub fn point_on_the_line(&self, t: f32) -> Point {
        let d = self.normalize_direction();
        let directional = d.scalar_mult(t);

        Point::new(
            self.origin.x + directional.x,
            self.origin.y + directional.y,
            self.origin.z + directional.z,
        )
    }
}
//direction is normalized- x^2 + y^2 + z^2 = 1 (direction)

#[test]
fn ray_test() {
    //let test_ray = default_vector();
}
