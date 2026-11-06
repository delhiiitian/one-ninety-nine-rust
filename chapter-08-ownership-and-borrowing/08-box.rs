// Chapter 8, Lesson 8: Box<T>, owning a value on the heap
// Run: cargo run --example ch08-08

// A family tree where each person may have children. A struct cannot
// contain itself directly (its size would be infinite), but it can contain
// a Box, which is a fixed-size pointer to a value on the heap.
#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Number(n) => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("{} * {}", show(a), show(b)),
    }
}

// A singly linked list: each node owns the next one through a Box.
struct Node {
    stop: &'static str,
    next: Option<Box<Node>>,
}

trait Greet {
    fn hello(&self) -> String;
}
struct English;
struct Hindi;
impl Greet for English {
    fn hello(&self) -> String {
        "Hello".into()
    }
}
impl Greet for Hindi {
    fn hello(&self) -> String {
        "Namaste".into()
    }
}

fn main() {
    // Box a single value: it now lives on the heap, owned by `boxed`.
    let boxed: Box<i32> = Box::new(42);
    println!("Boxed value: {}", *boxed + 1);

    // (2 + 3) * 4 as a tree of boxed expressions
    let expr = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Number(2.0)), Box::new(Expr::Number(3.0)))),
        Box::new(Expr::Number(4.0)),
    );
    println!("{} = {}", show(&expr), eval(&expr));

    // A metro line as a linked list
    let line = Node { stop: "Rajiv Chowk", next: Some(Box::new(Node { stop: "Mandi House", next: Some(Box::new(Node { stop: "Pragati Maidan", next: None })) })) };
    let mut current = Some(&line);
    while let Some(node) = current {
        print!("{} -> ", node.stop);
        current = node.next.as_deref();
    }
    println!("end");

    // Box<dyn Trait>: different types behind one pointer type (Chapter 4).
    let greeters: Vec<Box<dyn Greet>> = vec![Box::new(English), Box::new(Hindi)];
    for g in &greeters {
        println!("{}", g.hello());
    }
    println!("Expression debug: {expr:?}");
}
