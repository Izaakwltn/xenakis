use crate::points::build_point;
use std::Vec;

pub struct Polygon {
    //plane: Plane, // maybe
    points: Vec<Point>,
}

pub fn build_polygon(points: Vec) -> Polygon {
    Polygon { points }
}
// plane defined by polygon points

impl Polygon {
    pub fn polygon_plane(&self) -> Plane {}

    pub fn edge_cross_check(a: tuple, b: tuple) -> bool {
        let mut sh;
        let mut nsh;
        if a[1] >= 0.0 {
            sh = 1;
        } else if a[1] < 0 {
            sh = -1;
        }
        if b[1] >= 0 {
            nsh = 1;
        } else if b[1] < 0 {
            nsh = -1;
        }
        if sh == nsh {
            return false;
        } else if a[0] > 0.0 && b[0] > 0.0 {
            return true;
        } else if a[0] > 0.0 || b[0] > 0.0 {
            if (a[0] - (a[1] * (b[0] - a[0]) / (b[1] - a[1]))) > 0.0 {
                return true;
            }
        } else {
            return false;
        }
    }

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
        let mut crossings = 0;
        for i in 1..(new_coords.len() - 1) {
            if edge_cross_check(new_coords[i - 1], new_coords[i]) {
                crossings += 1;
            }
        }
        if crossings % 2 != 0 {
            return true;
        } else {
            return false;
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
    let test_plane = Plane(1.0, 2.0, 1.0, -2.0);
    let test_triangle = build_polygon(test_plane, test_points);
    let intersection_point = build_point(-2.0, -2.0, 4);
}
