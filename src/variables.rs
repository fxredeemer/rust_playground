pub fn variables(){

    let immutable = 12;
    // immutable = 22;
    println!("Immutable: {}", immutable);

    let mut mutable = 11;
    mutable += 12;
    println!("Mutable: {}", mutable);

    let hiding = 33;
    println!("Hiding: {}", hiding);

    let hiding = 77;
    println!("Hiding: {}", hiding);
    
    let hiding = 44;
    println!("Hiding: {}", hiding);
}