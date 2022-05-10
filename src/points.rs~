use crate::vectors::Vector;

//------------------------------------------------------------------------
//Point struct
//------------------------------------------------------------------------

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub fn build_point(x: f32, y: f32, z: f32) -> Point {
    Point { x, y, z }
}

pub fn default_point() -> Point {
    build_point(0.0, 1.0, 0.0)
}

impl Point {
    /*pub fn build_point(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn default_point() -> Point {
        Self::build_point(0.0, 1.0, 0.0)
    }*/

    fn copy(&self) -> Point {
        build_point(self.x, self.y, self.z)
    }

    fn point_to_vector(&self) -> Vector {
        Vector::build_vector(self.x, self.y, self.z)
    }
}

#[test]
fn point_test() {
    let test_point = default_point();
    assert_eq!(test_point.x, 0.0);
    assert_eq!(test_point.y, 1.0);
    assert_eq!(test_point.z, 0.0);

    let test_copy = test_point.copy();
    assert_eq!(test_copy.x, 0.0);
    assert_eq!(test_copy.y, 1.0);
    assert_eq!(test_copy.z, 0.0);

    let test_v = test_copy.point_to_vector();
    assert_eq!(test_v.x, 0.0);
    assert_eq!(test_v.y, 1.0);
    assert_eq!(test_v.z, 0.0);
}
