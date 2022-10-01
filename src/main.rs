fn main() {
    /***
     * Scaler data types in rust
     */
    //Integers
    //floating
    //character
    //boolean 

    //Tuple can be considered as a fixed sized array of different types
    //Tuple are zero indexed;

    let tup: (&str, i32) = ("Welcome to my channel", 45);
    let (channel, count) = tup;
    println!("Printing tuple using destructuring {} ande {}", channel, count);
    println!("Printing tuple using dot notation: {}", tup.1);

    //How to get data out of tuple 
    //1. Destructuring
    //2. Dot notation

    let  mut x:u32 = 6;
    println!("Hello, world! {}", x);

    x = 7;
    println!("Hello, world! {}", x);

    //constants must be type annotated cannot be inferred by compiler
    const NEW_SALARY:u32 = 10000;
    println!("Hello, world! {}", NEW_SALARY);
}
