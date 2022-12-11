fn main() {
    let point = (1, 0);

    if let (0, 0) = point {
        println!("Nullpunkt");
    } else if point.0 == 0 {
        println!("Auf der X-Achse: {}", point.1);
    } else if let (x, 0) = point {
        println!("Auf der Y-Aches: {}", x);
    }
}
