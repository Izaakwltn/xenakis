use crate::points::build_point;
use crate::points::default_point;
use crate::points::Point;
use crate::vectors::default_vector;
use crate::vectors::Vector;

//------------------------------------------------------------------------
//Rays
//------------------------------------------------------------------------

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    t_max: f32,
}

pub fn build_ray(origin: Point, direction: Vector, t_max: f32) -> Ray {
    Ray {
        origin,
        direction,
        t_max, //distance from the origin
    }
}

pub fn default_ray() -> Ray {
    build_ray(default_point(), default_vector(), 10000000000000000.0)
}

impl Copy for Ray {}

impl Clone for Ray {
    fn clone(&self) -> Ray {
        *self
    }
}
impl Ray {
    pub fn normalize_direction(&self) -> Vector {
        let magn = self.direction.length().sqrt();
        crate::vectors::build_vector(
            (self.direction.x / magn),
            (self.direction.y / magn),
            (self.direction.z / magn),
        )
    }

    pub fn point_on_the_line(&self, t: f32) -> Point {
        let d = self.normalize_direction();
        let directional = d.scalar_mult(t);

        build_point(
            self.origin.x + directional.x,
            self.origin.y + directional.y,
            self.origin.z + directional.z,
        )
    }
}
//direction is normalized- x^2 + y^2 + z^2 = 1 (direction)
