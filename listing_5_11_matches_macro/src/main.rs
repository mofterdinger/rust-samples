fn main() {
    let buchstabe = 'f';

    if matches!(buchstabe, 'A'..='Z' | 'a'..='z') {
        println!("Buchstabe");
    }
}
