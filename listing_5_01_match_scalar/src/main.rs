fn main() {
    let a = 3;

    match a {
        1 | 2 => println!("1 oder 2"),
        3 => println!("3"),
        3..=5 => {
            println!("Zwischen 3 und 5")
        }
        _ => println!("Anderer Wert"),
    }
}
