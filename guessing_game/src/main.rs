use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("a ver la maquinita que numero decidio?");

    let numero = rand::thread_rng().gen_range(1..1001);

    println!("elegi: {}", numero);

    println!("input dale.");

   


    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("no lei bien.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };
        
        println!("dijiste: {}", guess);

        match guess.cmp(&numero) {
            Ordering::Less => println!("arriba"),
            Ordering::Greater => println!("abajo"),
            Ordering::Equal => {
                println!("si");
                break;
            },
        }
    }
}
