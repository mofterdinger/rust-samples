use std::process::exit;

enum MyOption<T> {
    None,
    Some(T),
}

fn is_none<T>(opt: &MyOption<T>) -> bool {
    if let MyOption::<T>::None = opt {
        true
    } else {
        false
    }
}

fn expect<T>(opt: MyOption<T>) -> T {
    match opt {
        MyOption::None => {
            println!("Wir haben eine Problem ...");
            exit(1);
        }
        MyOption::Some(x) => x,
    }
}

fn main() {
    let opt = MyOption::Some(3.14_f32);
    let val = expect(opt);
    println!("{}", val);

    // let opt = MyOption::None; // Fehler
    let opt = MyOption::<i32>::None;

    if is_none(&opt) {
        println!("Enthält keinen Wert");
    }
    let val = expect(opt);
}
