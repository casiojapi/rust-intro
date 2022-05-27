fn main() {
    let x = 32;
    let es_par = x % 2 == 0;
    println!("EL NUMERO {} {} PAR.", x, if es_par {"es"} else {"no es"});

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 33 {
            break counter * 3 + 1
        }
    };

    println!("me dio 100? {}", if result == 100 {"si"} else {"no"});

    println!("\ncountdown:");
    let mut regre = 3;
    while regre > 0 {
        println!("{}!", regre);
        regre -= 1;
    }
    println!("despegue jaja");

    let a = [1, 2, 3, 5, 8, 13, 21];
    for x in a {
        print!("{}\t", x);
    }
    println!("");


    // for i in range

    for puto in (1..78).rev() {
        print!("{}! ", puto);
    }
    println!();

}
