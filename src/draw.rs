// Draw.rs

//figure out Polgon

//maybe image struct, takes

pub struct Image {
    pixels: Vec<Hue>,
}

impl Image {
    pub fn new(pixels: Vec<Hue>) -> Self {
        Self { pixels }
    }
    pub fn empty() -> Self {
        let v: Vec<Hue> = Vec::new();
        Image::new(v);
    }
}

fn dumb_draw(eye: Point, screen: Polygon, shapes: Vec<Box<dyn Shape>>) {
    let mut image = Image::empty();
    let r = Ray::new(eye, Vector::new(p.x, p.y, p.z), 10000000000.0);
    for shape in shapes {
        if shape.intersects(r) {
            image.push(shape.hue())
        }
    }
}
