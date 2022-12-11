fn main() {
    let feld = [1, 2, 3, 4, 5, -1];
    let mut i = 0;

    while let x @ 0..=i32::MAX = feld[i] {
        println!("{}", x);
        i += 1;
    }
}
