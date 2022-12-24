#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // let origin = Point { x: 0, y: 0.0 };

    let origin = Point { x: 0, y: 0 };
    let tfish: Point::<i32> = Point { ..origin };
    let fp_point = Point {
        x: 3.141_f32,
        y: 2.718,
    };
    println!("{:#?}, {:#?}", origin, fp_point);
}
