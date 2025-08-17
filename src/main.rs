use std::io;

fn main() {
    println!("Let's start a guess game");
    println!("Please input a guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read_line fail");

    println!("Your guess：{guess}");
}
