use crate::points;
use crate::vectors;

//------------------------------------------------------------------------
//Rays
//------------------------------------------------------------------------

pub struct Ray {
    origin: Point,
    direction: Vector,
    t_max: f32,
}

pub fn build_ray(origin: Point, direction: Vector, t_max: f64) -> Ray {
    Ray {
        origin,
        direction,
        t_max, //distance from the origin
    }
}

pub fn default_ray() -> Ray {
    build_ray(default_point(), default_vector(), 10000000000000000)
}
impl Ray {
    pub fn point_on_the_line(&self, t: f32) -> Point {
        let directional = self.direction.scalar_mult(t);

        build_point(
            self.origin.x + directional.x,
            self.origin.y + directional.y,
            self.origin.z + directional.z,
        )
    }
}
//direction is normalized- x^2 + y^2 + z^2 = 1 (direction)
