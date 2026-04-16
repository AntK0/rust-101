fn take_ownership(some_string: String) -> String {
    println!("{}", some_string);
    String::from("Goodbye, world!")
}

fn main() {
    let s = String::from("Hello, world!");
    let s = take_ownership(s);
    println!("{}", s);
}
