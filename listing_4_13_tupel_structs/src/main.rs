fn main() {
    struct Kilogramm(f64);
    struct Celsius(f64);

    let value = 42.1;
    let kg = Kilogramm(value);
    let temp = Celsius(value);

    println!("{},{},{}", value, kg.0, temp.0);
}
