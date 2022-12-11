#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rect {
    p0: Point,
    p1: Point,
}

fn main() {
    let (x, y) = (0, 0);
    let origin = Point { x, y };

    let p1 = Point { x: 10, ..origin };

    let rectangle = Rect { p0: origin, p1 };
    println!("{:#?}", rectangle);
}
