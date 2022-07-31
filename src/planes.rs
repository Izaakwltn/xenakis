use crate::points::Point;
use crate::rays::Ray;
use crate::vectors::Vector;

// Planes

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    fn abc_compliance(&self) {
        (self.a * self.a) + (self.b * self.b) + (self.c * self.c) == 0
    }

    fn origin_to_intersection_denominator(&self, ray: Ray) {
        let dir = ray.direction;
        crate::vectors::dot_product(self.a, dir.x)
            + crate::vectors::dot_product(self.b, dir.y)
            + crate::vectors::dot_product(self.c, dir.z)
    }

    fn not_in_path(&self, ray: Ray) {
        self.origin_to_intersection_denominator(ray) >= 0
    }

    //
    fn origin_to_intersection_numerator(&self, ray: Ray) {
        let o = ray.origin;
        -(self.a * o.x + self.b * o.y + self.c * o.z + self.d)
    }

    fn origin_to_intersection(&self, ray: Ray) {
        self.origin_to_intersection_numerator(ray) / self.origin_to_intersection_denominator(ray)
    }

    //
    fn plane_in_front(&self, ray: Ray) {
        self.origin_to_intersection >= 0
    }
}

//A * x + B * y + C * z + D = 0, D
