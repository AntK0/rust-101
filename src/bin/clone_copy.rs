#[derive(Debug, Clone, Copy)] // copy is an enchancement of clone, it allows for implicit copying instead of moving
struct ImplicitDuplication {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct ExplicitDuplication{
    s: String,
} 

fn main() {
    let p1 = ImplicitDuplication{ x: 1, y: 2 };
    let p2 = p1; // This is a copy, not a move, because Point implements Copy
    println!("p1: {:?}, p2: {:?}", p1, p2);

    let w1 = ExplicitDuplication{ s: String::from("Hello") };
    let w2 = w1.clone(); // This is a copy, not a move, because WithHeapData implements Clone
    println!("w1: {:?}, w2: {:?}", w1, w2);
}