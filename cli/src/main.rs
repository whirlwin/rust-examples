fn main() {
    let pattern = std::env::args().nth(1).expect("No first argument given");
    let path = std::env::args().nth(2).expect("No second argument given");

    println!("Fist arg: {:?}, second arg: {:?}", pattern, path)
}
