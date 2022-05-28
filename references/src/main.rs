fn main() {
    let mut s1 = String::from("LA CONCHA DE TU MADRE");
    println!("antes de la funcion: {}", s1);
    hacer_algo_con_el_string(&s1);
    println!("mira ahora sigue andando: {}", s1);

    println!("\n\n#########\n\n");
    println!("antes de la funcion: {}", s1);
    change(&mut s1); // para llamar a la referencia mutable el string TIENE que ser mutable... ddddd
    println!("post change(): {}", s1);

    
}


fn hacer_algo_con_el_string(esteerre: &String) {
    println!("mira ahora si: {}", esteerre);        // NO TIENE OWNERSHIP PORQUE ES UNA REFERENCIA - O SEA NO ES UN MOVE
}

// si quiero cambiar algo por referencia -> recibo un &mut String (o sea, referencia a string mutable)
fn change(some: &mut String) {
    some.push_str(" :: MIRA COMO TE APENDEO, FORRO.")
}

// importante: 
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.