fn main() {
    struct Point {
        x: i32,
        y: i32,
    }
    struct Rect {
        p1: Point,
        p2: Point,
    }

    let origin = Point { x: 0, y: 0 };
    let r = Rect {
        p1: origin,
        p2: Point { x: 1, y: 1 },
    };

    match r {
        Rect { p1, p2 } if p2.x - p1.x == p2.y - p1.y => println!("Ist ein Quadrat"),
        Rect {
            p1: Point { x: 0, y: 0 },
            ..
        } => println!("Beginnt am Nullpunkt"),
        _ => println!("Keine Übereinstimmung"),
    }
}
