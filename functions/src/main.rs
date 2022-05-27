fn main() {
    println!("Hello, world!");

    another_function(1-32);
    pelotudo(32.2, 'c');
    let x = five();
    println!("THE VALUE IS: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn pelotudo(temp: f32, scale: char) {
    println!("temperaturA ES: {} {}", temp, scale.to_uppercase())
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}