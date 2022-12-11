fn main() {
    #[derive(Debug)]
    enum Wert {
        Ganzahl(i32),
        Fliesskomma(f64),
        Tupel((i32, i64)),
    }

    let wert = Wert::Ganzahl(3);

    println!("Der Wert ist {:?}", wert);
}
