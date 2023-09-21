fn main() {
    scope_example();
    string_types();
    ownership();
    move_example();
    ovnership_and_functions();
    moving_return_values();
}

fn scope_example() {
    //no hoisting: s is not valid here
    let s: &str = "Hello, world!"; //s is a string literal
    println!("{}", s);
} //end of scope, s is no longer valid

fn string_types() {
    //&str
    let literal = "String literal: is of a known size and it's stored on stack.";

    //String
    let object = String::from("String object: can be modified and it's stored on heap.");

    println!("{}", literal);
    println!("{}", object);
}

fn ownership() {
    let mut hw = String::from("Hello");
    hw.push_str(", ");
    hw.push_str("World!");

    println!("{}", hw);
}

fn move_example() {
    let x = 44;
    let y = x; //integer are simple values, no move

    println!("The x is: {}", x);
    println!("The y is: {}", y);

    let foo = String::from("foo");
    let bar = foo; //move, foo is no longer valid pointer

    //println!("{}", foo); //illegal: value of foo is moved
    println!("{}", bar);

    let baz = String::from("si vic pacem para bellum");
    let baq = baz.clone(); //heads up: cloning can be expensive!

    println!("{}", baz); //legal: baz is cloned, not moved
    println!("{}", baq);
}

fn ovnership_and_functions() {
    let text = String::from("some text to be moved");
    takes_ownership(text); //text's value is moved to the function
                           //println!("{}", text); //illegal: text's value is already moved

    let num = 777;
    makes_copy(num); //copy of num is passed to fn
    println!("Still available: {}", num);
}

fn takes_ownership(t: String) {
    println!("{}", t);
} //t is droped, the backing memory is freed.

fn makes_copy(t: i32) {
    println!("Copied value: {}", t);
} //t goes out of scope

fn moving_return_values() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2); //s2 is moved
    let (s4, len) = calculate_length(s3);

    println!("{}", s1);
    println!("{}, length: {}", s4, len);
}

fn gives_ownership() -> String {
    let s = String::from("Transfered value!");

    s //s is returned to calling fn, so ownership is moved to that fn
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// example of function which returns both result and value taken from calling function
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}
