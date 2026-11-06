// Chapter 4, Lesson 8: Traits, shared behaviour
// Run: cargo run --example ch04-08

// A trait lists methods that a type promises to provide.
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    // A default method: types get it for free, but may override it.
    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

struct Circle {
    radius: f64,
}

struct Rect {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> String {
        "Circle".to_string()
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> String {
        "Rectangle".to_string()
    }
    fn describe(&self) -> String {
        format!("{}x{} rectangle", self.width, self.height) // overridden
    }
}

// Static dispatch: works with any one type that implements Shape.
fn print_shape(shape: &impl Shape) {
    println!("-> {}", shape.describe());
}

fn main() {
    let c = Circle { radius: 2.0 };
    let r = Rect { width: 3.0, height: 4.0 };
    print_shape(&c);
    print_shape(&r);

    // Dynamic dispatch: a list of DIFFERENT types that share a trait.
    // Box<dyn Shape> means "some shape, decided at run time".
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rect { width: 2.0, height: 5.0 }),
        Box::new(Circle { radius: 0.5 }),
    ];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("{} shapes, total area {total:.2}", shapes.len());
}
