//------------------------------------------------------------------------
//Rays
//------------------------------------------------------------------------

pub struct Ray {
    origin: Point,
    direction: Vector,
    t_max: f32,
}

pub fn build_ray(origin: Point, direction: Vector, t_max: f32) -> Ray {
    Ray {
        origin,
        direction,
        t_max,
    }
}

pub fn default_ray() {}
//direction is normalized- x^2 + y^2 + z^2 = 1 (direction)
