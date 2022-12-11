struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 10 };

    let Point { x: breite, y: _ } = p1; // let breite = p1.x

    println!("{}", breite)
}
