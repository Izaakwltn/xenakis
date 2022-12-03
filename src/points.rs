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

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}
impl Point {
    pub fn copy(&self) -> Point {
        build_point(self.x, self.y, self.z)
    }

    pub fn to_vector(&self) -> Vector {
        crate::vectors::build_vector(self.x, self.y, self.z)
    }
    pub fn print(&self) {
        println!("{:.32}, {:.32}, {:.32}", self.x, self.y, self.z)
    }
}

pub fn point_subtract(a: Point, b: Point) -> Point {
    build_point(a.x - b.x, a.y - b.y, a.z - b.z)
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

    let test_v = test_copy.to_vector();
    assert_eq!(test_v.x, 0.0);
    assert_eq!(test_v.y, 1.0);
    assert_eq!(test_v.z, 0.0);
}
