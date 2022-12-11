fn main() {
    let t = (3, 4);

    let ausgabe = match t {
        (3, _) => "Wert is 3",
        (_, y @ 3..=5) if y < 5 => "Zweiter Wert ist 3-4",
        _ => "Wert ist anders",
    };

    println!("{}", ausgabe);
}
