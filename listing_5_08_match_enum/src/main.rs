fn main() {
    enum Wert {
        Ganzahl(i32),
        Tupel((i32, i64)),
    }

    let wert = Wert::Ganzahl(3);

    match wert {
        Wert::Ganzahl(a) => println!("Zahl: {}", a),
        Wert::Tupel((x, _)) => println!("Tupel"),
    }
}
