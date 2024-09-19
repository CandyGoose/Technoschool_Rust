use std::f64;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.get_x();
        let dy = self.y - other.get_y();
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(7.0, 1.0);

    let distance = point1.distance(&point2);

    println!("The distance between the two points is: {:.2}", distance);
}
