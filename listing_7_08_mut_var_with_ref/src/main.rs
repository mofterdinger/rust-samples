struct CloneMe {
    x: i32,
}

fn ausgabe_clone_me(reference: &CloneMe) {
    println!("{}", reference.x);
}

fn main() {
    let mut val = CloneMe { x: 1 };
    val.x = 2;
    let ref1 = &val;
    // val.x = 3; // Fehler in der nächsten Zeile
    ausgabe_clone_me(ref1);
    // val.x = 3; // Fehler in der nächsten Zeile
    let ref2 = ref1;
    ausgabe_clone_me(&val);
    // val.x = 3; // Fehler in der nächsten Zeile
    ausgabe_clone_me(ref2);
    val.x = 3;
}
