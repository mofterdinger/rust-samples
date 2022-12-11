fn main() {
    #[derive(Debug)]
    enum Ampel {
        Rot,
        Gelb,
        Grün,
    }

    let ampel = Ampel::Grün;

    println!("Die Ampel steht auf {:?}", ampel);
}
