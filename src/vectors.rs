use crate::points::Point;

//------------------------------------------------------------------------
//Vector struct
//------------------------------------------------------------------------

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn build_vector(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn default_vector() -> Vector {
        Self::build_vector(0.0, 1.0, 0.0)
    }

    pub fn copy(&self) -> Vector {
        Self::build_vector(self.x, self.y, self.z)
    }

    pub fn vector_to_point(&self) -> Point {
        crate::points::build_point(self.x, self.y, self.z)
    }

    pub fn length(&self) -> f32 {
        //otherwise known as magnitude
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn scalar_mult(&self, n: f32) -> Vector {
        Self::build_vector(self.x * n, self.y * n, self.z * n)
    }
}

//------------------------------------------------------------------------
//Other Vector Calculations
//------------------------------------------------------------------------
impl Vector {
    pub fn vect_add(a: Vector, b: Vector) -> Vector {
        Self::build_vector(a.x + b.x, a.y + b.y, a.z + b.z)
    }

    pub fn vect_subtract(a: Vector, b: Vector) -> Vector {
        Self::build_vector(a.x - b.x, a.y - b.y, a.z - b.z)
    }

    pub fn dot_product(a: Vector, b: Vector) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }

    pub fn cross_product(a: Vector, b: Vector) -> Vector {
        Vector {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }
}

#[test]
fn vector_test() {
    let test_vector = Vector::default_vector();
    assert_eq!(test_vector.x, 0.0);
    assert_eq!(test_vector.y, 1.0);
    assert_eq!(test_vector.z, 0.0);

    let test_copy = test_vector.copy();
    assert_eq!(test_copy.x, 0.0);
    assert_eq!(test_copy.y, 1.0);
    assert_eq!(test_copy.z, 0.0);

    //let test_v = test_copy.point_to_vector();
    //assert_eq!(test_v.x, 0.0);
    //assert_eq!(test_v.y, 1.0);
    //assert_eq!(test_v.z, 0.0);
}