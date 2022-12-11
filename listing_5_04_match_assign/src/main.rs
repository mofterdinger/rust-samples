fn main() {
    let a = 3;

    let ausgabe = match a {
        innen if innen < 5 => innen + a,
        innen => innen * a,
    };

    println!("{}", ausgabe);
}
