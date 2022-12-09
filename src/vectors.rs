use crate::points::Point;

//------------------------------------------------------------------------
//Vector struct
//------------------------------------------------------------------------

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

//making new Vectors
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

impl Copy for Vector {}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        *self
    }
}

//Converting Vector to Point
impl Vector {
    pub fn to_point(&self) -> Point {
        crate::points::Point::new(self.x, self.y, self.z)
    }
}

//Converting Point to Vector
impl Point {
    pub fn to_vector(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
}

impl Vector {
    pub fn length(&self) -> f32 {
        //otherwise known as magnitude
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn scalar_mult(&self, n: f32) -> Vector {
        Vector::new(self.x * n, self.y * n, self.z * n)
    }
}

//------------------------------------------------------------------------
//Calculations with two Vectors
//------------------------------------------------------------------------

pub fn vect_add(a: Vector, b: Vector) -> Vector {
    Vector::new(a.x + b.x, a.y + b.y, a.z + b.z)
}

pub fn vect_subtract(a: Vector, b: Vector) -> Vector {
    Vector::new(a.x - b.x, a.y - b.y, a.z - b.z)
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

// Vector tests
#[test]
fn vector_test() {
    let test_vector = Vector::default();
    assert_eq!(test_vector.x, 0.0);
    assert_eq!(test_vector.y, 1.0);
    assert_eq!(test_vector.z, 0.0);

    let sample_vector = Vector::new(1.0, 2.0, 4.0);
    println!("{:.32}", sample_vector.length());

    let test_point = test_vector.to_point();
    assert_eq!(test_point.x, 0.0);
    assert_eq!(test_point.y, 1.0);
    assert_eq!(test_point.z, 0.0);
}
