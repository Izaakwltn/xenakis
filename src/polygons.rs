use crate::points::build_point;
use std::Vec;

pub struct Polygon {
    points: Vec<Point>,
}

pub fn build_polygon(points: Vec) -> Polygon {
    Polygon { points }
}
// plane defined by polygon points

impl Polygon {
    pub fn polygon_plane(&self) -> Plane {}
    pub fn inside(&self, intersection_point: Point) -> bool {
        let mut xz_coords = Vec::new();
        for i in self.Points {
            xz_coords.push((i.x, i.z));
        }
        let new_origin = (intersection_point.x, intersection_point.z);
        let mut new_coords = Vec::new();
        for i in xz_coords {
            new_coords.push((i[0] - new_origin[0], i[1] - new_origin[1]));
        }
    }
}

#[test]
fn polygon_test() {
    let test_points = vec![
        build_point(-3.0, -3.0, 7.0),
        build_point(3.0, -4.0, 3.0),
        build_point(4, -5, 4.0),
    ];
    let test_triangle = build_polygon(test_points);
    let intersection_point = build_point(-2.0, -2.0, 4);
}