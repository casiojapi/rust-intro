use std::io;

fn main() {
    println!("a ver la maquinita que numero decidio?");

    println!("input dale.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("no lei bien.");
    println!("dijiste: {}", guess);
}
