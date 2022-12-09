use crate::points::Point;
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::vectors::Vector;

// Planes

#[derive(Debug)]
pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    //hue: Hue,
}

//Making new Planes
impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }
    pub fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Plane {
    fn abc_compliance(&self) -> bool {
        (self.a * self.a) + (self.b * self.b) + (self.c * self.c) == 0.0
    }

    fn origin_to_intersection_denominator(&self, ray: Ray) -> f32 {
        crate::vectors::dot_product(Vector::new(self.a, self.b, self.c), ray.direction)
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
}

impl Shape for Plane {
    fn intersects(&self, ray: Ray) -> bool {
        let vd = self.origin_to_intersection_denominator(ray);
        if vd <= 0.0 {
            false
        } else {
            let vo = self.origin_to_intersection_numerator(ray);
            vo / vd > 0.0 //expressions are cool
        }
    }

    fn intersection_point(&self, ray: Ray) -> Point {
        let vd = self.origin_to_intersection_denominator(ray);
        let t = self.origin_to_intersection_numerator(ray) / vd;
        let o = ray.origin;
        Point::new(o.x + vd * t, o.y + vd * t, o.z + vd * t)
    }
}

#[test]
fn plane_test() {
    let test_plane = Plane::new(1.0, 0.0, 0.0, -7.0);
    let test_ray = Ray::new(
        Point::new(2.0, 3.0, 4.0),
        Vector::new(0.577, 0.577, 0.577),
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
