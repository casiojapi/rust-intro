fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // tuplas
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // arrays, fixed size (stack). Vector, es posta (en heap)

    let a = [3; 5];
    println!("let a = [3; 5];");
    let mut i = 0;
    print!("[");
    while (i < 5) {
        print!(" (index: {}, value: {})",i, a[i]);
        i += 1;
    }
    println!("]");
    // 
}
