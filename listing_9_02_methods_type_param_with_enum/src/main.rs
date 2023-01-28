use std::process::exit;

enum MyOption<T> {
    None,
    Some(T),
}

impl<T> MyOption<T> {
    fn is_none(&self) -> bool {
        if let MyOption::None = self {
            true
        } else {
            false
        }
    }

    fn expect(self) -> T {
        match self {
            MyOption::None => {
                println!("Wir haben eine Problem ...");
                exit(1);
            }
            MyOption::Some(x) => x,
        }
    }
}

fn main() {
    let opt = MyOption::Some(3.141_f32);
    let val = opt.expect();
    println!("{}", val);

    // let opt = MyOption::None; // Fehler
    let opt = MyOption::<i32>::None;

    if opt.is_none() {
        println!("Enthält keinen Wert");
    }
    let val = opt.expect();
}
