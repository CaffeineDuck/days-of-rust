// More about control statements here: https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    // Declaring a varible named `a` is equal to 10
    let a = 10;
    
    // This block of code is run if the value of a
    // is equal to the integer 10
    if a == 10 {
        println!("A is 10");
    }
    // This block of code is run if the value of a 
    // is not equal to 100
    else if a != 11 {
        println!("A is 11");
    }
    // This block of code is run if both of the previous 
    // conditions weren't matched
    else {
        println!("Noone know the value of A");
    }
}