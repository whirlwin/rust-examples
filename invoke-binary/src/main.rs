use std::process::Command;

fn main() {
    let output = Command::new("openssl")
        .arg("rand")
        .arg("-base64")
        .arg("12")
        .output()
        .expect("failed to execute process");
    println!("Random string: {}", String::from_utf8(output.stdout).expect("Failed to convert to string"));
}
