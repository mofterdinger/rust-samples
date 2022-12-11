fn main() {
    let a = 3;

    let ausgabe = match a {
        1 | 2 => "1 oder 2",
        3 => "3",
        3..=5 => "Zwischen 3 und 5",
        _ => "Anderer Wert",
    };

    println!("{}", ausgabe);
}
