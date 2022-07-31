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
    //Finds the vector from the ray origin to the sphere center
    fn origin_to_center(&self, ray: Ray) -> Vector {
        point_subtract(self.center, ray.origin).to_vector()
        // crate::rays::build_ray(ray.origin, oc, ray.t_max)
    }

    //Checks whether the ray origin is outside the sphere
    fn origin_outside_sphere(&self, ray: Ray) -> bool {
        let oc = self.origin_to_center(ray);
        dot_product(oc, oc) > self.radius_squared
    }

    // Finds the ray's closest approach to the sphere
    fn closest_approach(&self, ray: Ray) -> f32 {
        let oc = self.origin_to_center(ray);
        dot_product(oc, ray.normalize_direction())
    }

    // checks whether the sphere is in front of the ray
    fn sphere_in_front(&self, ray: Ray) -> bool {
        self.closest_approach(ray) > 0.0
    }

    // Checks the distance from the closest approach to the sphere's center
    fn half_chord_distance_squared(&self, ray: Ray) -> f32 {
        let ca = self.closest_approach(ray);
        let oc = self.origin_to_center(ray).length();
        self.radius_squared - ((oc * oc) - (ca * ca))
    }

    fn intersection_distance(&self, ray: Ray) -> f32 {
        self.closest_approach(ray) - self.half_chord_distance_squared(ray).sqrt()
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        if (self.origin_outside_sphere(ray)) {
            return false;
        } else if (self.sphere_in_front(ray)) {
            return true;
        }
        return false;
    }

    pub fn intersection_point(&self, ray: Ray) -> Point {
        let origin = ray.origin;
        let dir = ray.normalize_direction();
        let int_dist = self.intersection_distance(ray);
        crate::points::build_point(
            origin.x + (dir.x * int_dist),
            origin.y + (dir.y * int_dist),
            origin.z + (dir.z * int_dist),
        )
    }
    pub fn unit_vector_normal(&self, ray: Ray) -> Vector {
        let i = self.intersection_point(ray);
        let c = self.center;
        let r = self.radius;
        crate::vectors::build_vector((i.x - c.x) / r, (i.y - c.y) / r, (i.z - c.z) / r)
    }
}

#[test]
fn sphere_test() {
    let test_ray = crate::rays::build_ray(
        crate::points::build_point(1.0, -2.0, -1.0),
        crate::vectors::build_vector(1.0, 2.0, 4.0),
        10000000000000000.0,
    );
    let test_sphere = build_sphere(crate::points::build_point(3.0, 0.0, 5.0), 3.0);
    let normalized = test_ray.normalize_direction();
    println!(
        "Normalized Ray Direction: {} {} {}",
        normalized.x, normalized.y, normalized.z
    );
    let origin_check = test_sphere.origin_outside_sphere(test_ray);
    println!("Does the ray originate inside the sphere? {}", origin_check);

    let ca = test_sphere.closest_approach(test_ray);
    println!("Closest Approach: {:.32}", ca);
    let sphere_front_check = test_sphere.sphere_in_front(test_ray);
    println!(
        "Is the sphere in front of the origin? {}",
        sphere_front_check
    );
    let half_chord_check = test_sphere.half_chord_distance_squared(test_ray);
    println!("Half Chord Distance Squared: {}", half_chord_check);
    let intersection_distance_check = test_sphere.intersection_distance(test_ray);
    println!("intersection distance: {:.32}", intersection_distance_check);
    let intersection_point_check = test_sphere.intersection_point(test_ray);
    println!(
        "Intersection point {} {} {}",
        intersection_point_check.x, intersection_point_check.y, intersection_point_check.z
    );
    let unit_vector_check = test_sphere.unit_vector_normal(test_ray);
    println!(
        "Unit Vector Normal: [{} {} {}]",
        unit_vector_check.x, unit_vector_check.y, unit_vector_check.z
    );
}
