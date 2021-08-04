// More about structs here: https://doc.rust-lang.org/book/ch05-01-defining-structs.html

// Using #[derive(Debug)] derives the debug trairt
// for the struct so that it can be used in the println!
#[derive(Debug)]

// Creating a strcut named Person
struct Person {
    name: String,
    age: u32,
}


// Adding methods to the struct Person
impl Person {
    // Creating a is_avi method to check if a 
    // the provided person is avi
    // This can be accessed using `person_instance.is_avi()`
    fn is_avi(&self) -> bool {
        // Comparing with the string avi
        if self.name == String::from("avi") {
            // Returning true
            return true;
        }
        // Returning false
        false
    }

    // This is a function to create new Pesron object 
    // which can be accessed using `Person::new()`
    fn new(name: String, age: u32) -> Person {
        // Creating and returning a Person object
        Person { name, age }
    }
}

fn main() {
    // Creating a new instance of Person struct
    let person1 = Person {
        name: String::from("avi"),
        age: 16,
    };

    // Creating another instance of Person struct 
    // using the function in the struct
    let person2 = Person::new(String::from("samrid"), 15);

    // Using the methods in the struct
    let person1_is_avi = person1.is_avi();
    let person2_is_avi = person2.is_avi();

    // Printing the values
    println!("{:?} is avi? {}", person1, person1_is_avi);
    println!("{:?} is avi? {}", person2, person2_is_avi);
}
