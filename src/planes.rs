use crate::points::build_point;
use crate::points::Point;
use crate::rays::build_ray;
use crate::rays::Ray;
use crate::vectors::build_vector;
use crate::vectors::Vector;

// Planes

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

pub fn build_plane(a: f32, b: f32, c: f32, d: f32) -> Plane {
    Plane { a, b, c, d }
}

pub fn default_plane() -> Plane {
    build_plane(0.0, 0.0, 0.0, 0.0)
}

impl Plane {
    fn abc_compliance(&self) -> bool {
        (self.a * self.a) + (self.b * self.b) + (self.c * self.c) == 0.0
    }

    fn origin_to_intersection_denominator(&self, ray: Ray) -> f32 {
        crate::vectors::dot_product(build_vector(self.a, self.b, self.c), ray.direction)
    }

    fn not_in_path(&self, ray: Ray) -> bool {
        self.origin_to_intersection_denominator(ray) >= 0.0
    }

    //
    fn origin_to_intersection_numerator(&self, ray: Ray) -> f32 {
        let o = ray.origin;
        -(self.a * o.x + self.b * o.y + self.c * o.z + self.d)
    }

    fn origin_to_intersection(&self, ray: Ray) -> f32 {
        //distance
        self.origin_to_intersection_numerator(ray) / self.origin_to_intersection_denominator(ray)
    }

    //
    fn plane_in_front(&self, ray: Ray) -> bool {
        self.origin_to_intersection(ray) >= 0.0
    }

    // instead of doing it all as one, functional item, do the checks and then the intersection in a function with side effects
    pub fn intersection_point(&self, ray: Ray) -> Point {
        let vd = self.origin_to_intersection_denominator(ray);
        let t = self.origin_to_intersection_numerator(ray) / vd;
        let o = ray.origin;
        build_point(o.x + vd * t, o.y + vd * t, o.z + vd * t)
    }
}

#[test]
fn plane_test() {
    let test_plane = build_plane(1.0, 0.0, 0.0, -7.0);
    let test_ray = build_ray(
        build_point(2.0, 3.0, 4.0),
        build_vector(0.577, 0.577, 0.577),
        10000000000000000.0,
    );
    assert!(test_plane.not_in_path(test_ray));
    assert!(test_plane.plane_in_front(test_ray));
    assert_eq!(
        test_plane.origin_to_intersection_denominator(test_ray),
        0.577
    );
    assert_eq!(test_plane.origin_to_intersection_numerator(test_ray), 5.0);
    assert_eq!(
        test_plane.origin_to_intersection_numerator(test_ray)
            / test_plane.origin_to_intersection_denominator(test_ray),
        8.665511
    );
    let i = test_plane.intersection_point(test_ray);
    assert_eq!(i.x, 7.0);
    assert_eq!(i.y, 8.0);
    assert_eq!(i.z, 9.0);
}
