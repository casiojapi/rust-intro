fn main() {
    println!("Hello, world!");

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); -> no anda. Cuando copias algo que esta en heap, no copias la data, si no que tenes un nuevo puntero al mismo heap, y el puntero viejo muere. (para evitar double free)
    println!("{}", s2);     // esto si anda, ya que el s1 esta muerto pero s2 ahora vive

    // por eso a let s2 = s1 se le dice "move" - en si es algo asi como un "shallow copy"

    // si queremos un deep copy (2 versiones separadas, memorias distintas, punteros distintas - mismos datos apuntados respectivamente)
    let ss1 = String::from("deep");
    let ss2 = ss1.clone();

    println!("ss1: {},\tss2: {}", ss1, ss2);


    print!("\n\n\n###############################\n\n\n");
    let stro = String::from("stro");
    println!("before pasing \'stro\' into a function: {}", stro);
    take_ownership(stro);
   // println!("after pasing \'stro\' into a function: {}", stro); -> no se puede porque la memoria la movi a "a_string" de takes_ownership. Entonces cuando returnea la funcion, como la movi a "a_string", termina su scope y libera la memoria. Por ende cuando trato de volver a acceder a 'stro' no puedo, movi y libere la memoria.
   // de verdad me gusta esto? no se. Parece un garbage collector pero mas rompe huevos. Espero que haya un gran cambio con respecto a performance, porque si no alta paja acostumbrarse a laburar asi. Aunque debe instalarte una intuicion de buenas practicas de manejo de memoria(????????????? si..... no?
    print!("\n");

    let equis = 5;
    println!("before pasing \'equis\' into a function: {}", equis);
    makes_copy(equis);
    println!("after pasing \'equis\' into a function: {}", equis);


    print!("\n\n\n###############################\n\n\n");
    println!("a tratar de solucionar esto para ver si puedo mantener la data del string en caso de que la necesite despues de mandarla a una funcion. (porque no la necesitaria?)\n\n");

    let sss1 = gives_ownership(); // le paso el ownership a algo creado dentro una funcion a sss1
    println!("sss1: {}", sss1);
    let sss2 = String::from("CHAU");    // s2 entra en scope
    println!("sss2: {}", sss2);
    let sss3 = takes_and_gives_back(sss2);  // s2 entra a la funcion, y lo que devuelve (return) se lo mueve a sss3
    println!("sss3: {}", sss3);
}

fn take_ownership(a_string: String) {
    println!("adentro de take ownership \'stro\' es: {}", a_string);
}

fn makes_copy(un_int: i32) {
    println!("adentro de makes copy \'equis\' es: {}", un_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("HOLA");
    some_string
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}