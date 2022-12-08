//------------------------------------------------------------------------
//Point struct
//------------------------------------------------------------------------

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl Point {
    pub fn print(&self) {
        println!("{:?}", self)
    }
}

pub fn point_subtract(a: Point, b: Point) -> Point {
    Point::new(a.x - b.x, a.y - b.y, a.z - b.z)
}
#[test]
fn point_test() {
    let test_point = Point::default();
    println!("{:?}", test_point);
    assert_eq!(test_point.x, 0.0);
    assert_eq!(test_point.y, 1.0);
    assert_eq!(test_point.z, 0.0);
}
