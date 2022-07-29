use crate::points::point_subtract;
//use crate::points::to_vector;
use crate::points::Point;
use crate::rays::Ray;
use crate::vectors::dot_product;
use crate::vectors::Vector;

//------------------------------------------------------------------------
//General Shape functions
//------------------------------------------------------------------------

//------------------------------------------------------------------------
//Spheres
//------------------------------------------------------------------------

pub struct Sphere {
    center: Point,
    radius: f32,
    radius_squared: f32,
}

pub fn build_sphere(center: Point, radius: f32) -> Sphere {
    Sphere {
        center,
        radius,
        radius_squared: radius * radius,
    }
}

impl Sphere {
    /*pub fn slow_is_intersection(&self, ray: Ray) -> bool {
        let a = ray.normalize_direction();
        let b = 2.0
            * ((a.x * (ray.origin.x + self.center.x))
                + (a.y * (ray.origin.x + self.center.x))
                + (a.z * (ray.origin.z + self.center.z)));
        let c = (ray.origin.x - self.center.x).powf(2.0)
            + (ray.origin.y - self.center.y).powf(2.0)
            + (ray.origin.z - self.center.y).powf(2.0)
            - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * c;

        discriminant > 0.0
    }
    pub fn slow_calculate_intersection(&self, ray: Ray) -> Point {
        let a = ray.normalize_direction();
        let b = 2.0
            * ((a.x * (ray.origin.x + self.center.x))
                + (a.y * (ray.origin.x + self.center.x))
                + (a.z * (ray.origin.z + self.center.z)));
        let c = (ray.origin.x - self.center.x).powf(2.0)
            + (ray.origin.y - self.center.y).powf(2.0)
            + (ray.origin.z - self.center.y).powf(2.0)
            - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * c;

        let t = (-b - discriminant.sqrt()) / 2.0;

        ray.point_on_the_line(t)
    }*/

    pub fn origin_inside_sphere(&self, ray: Ray) -> bool {
        let oc = point_subtract(self.center.copy(), ray.origin.copy()).to_vector();
        dot_product(oc, oc) < self.radius_squared
    }

    pub fn closest_approach(&self, ray: Ray) -> f32 {
        let oc = point_subtract(self.center.copy(), ray.origin.copy()).to_vector();
        dot_product(oc, ray.normalize_direction())
    }

    pub fn half_chord_distance_squared(&self, closest_approach: f32) -> f32 {
        self.radius_squared * (closest_approach * closest_approach)
    }
}

#[test]
fn sphere_test() {
    let test_ray = crate::rays::build_ray(
        crate::points::build_point(1.0, -2.0, -1.0),
        crate::vectors::build_vector(1.0, 2.0, 4.0),
        10000000000000000.0,
    );
    let normalized = test_ray.normalize_direction();
    println!("{} {} {}", normalized.x, normalized.y, normalized.z);
}

//maybe make is_intersection for each shape as impl function

/*pub fn is_intersection(sphere: Sphere, ray: Ray) -> bool {
    let A = ray.normalize_direction();
    let B = 2 * (A.x * (ray.origin.x + sphere.center
}
*/
