fn main() {
    let a = 3;

    let ausgabe = match a {
        innen @ 0..=4 => innen + a,
        innen => innen * a,
    };

    println!("{}", ausgabe);
}
