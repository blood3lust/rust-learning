use std::io;
fn main() {
    println!("Как тебя зовут?");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Не удалось прочитать строку...");
    let name = name.trim();
    println!("Привет, {name}");
}
