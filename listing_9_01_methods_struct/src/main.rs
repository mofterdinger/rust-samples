struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Point {
    fn dist_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn dist(self: &Point, other: &Point) -> f64 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        (diff_x.powi(2) + diff_y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("Distance to 0, 0: {}", p1.dist_to_origin());

    let p2 = Point::new(9.0, 12.0);
    println!("Distance: {}", p1.dist(&p2));
}
