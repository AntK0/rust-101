#[derive(Debug)]
struct CustomClone {
    copyable: String,
    non_copyable: String,
}

impl Clone for CustomClone {
    fn clone(&self) -> Self {
        CustomClone {
            copyable: self.copyable.clone(),
            non_copyable: String::from("This is a new string, not a clone of the original"),
        }
    }
}

fn main() {
    let original = CustomClone {
        copyable: String::from("This is a cloneable string"),
        non_copyable: String::from("This is a non-cloneable string"),
    };

    let cloned = original.clone();

    println!("Original: {:?}", original);
    println!("Cloned: {:?}", cloned);
}