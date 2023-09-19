fn main() {
    scope_example();
    string_types();
    ownership();
}

fn scope_example() {
    //no hoisting: s is not valid here
    let s: &str  = "Hello, world!"; //s is a string literal
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