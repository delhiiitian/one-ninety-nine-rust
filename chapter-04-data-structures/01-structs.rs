// Chapter 4, Lesson 1: Structs
// Run: cargo run --example ch04-01

// A struct groups related values under one name, each with a field name.
struct Student {
    name: String,
    roll_no: u32,
    marks: f32,
    is_active: bool,
}

// A tuple struct: fields have no names, only positions.
struct Rgb(u8, u8, u8);

// A unit struct has no fields at all (useful as a marker, see traits).
struct Admin;

fn main() {
    // Create an instance by filling in every field.
    let mut riya = Student {
        name: String::from("Riya Sharma"),
        roll_no: 101,
        marks: 88.5,
        is_active: true,
    };
    println!("{} (roll {}) scored {}", riya.name, riya.roll_no, riya.marks);

    // Change a field (the whole variable must be mut).
    riya.marks += 2.0;
    println!("After re-evaluation: {}", riya.marks);

    // Field init shorthand: when a variable has the field's name.
    let name = String::from("Arjun Rao");
    let roll_no = 102;
    let arjun = Student { name, roll_no, marks: 76.0, is_active: true };

    // Struct update syntax: copy the remaining fields from another value.
    let former = Student { name: String::from("Old Student"), is_active: false, ..arjun };
    println!("{} active? {} (marks copied: {})", former.name, former.is_active, former.marks);

    // Destructuring a struct
    let Student { name, marks, .. } = riya;
    println!("Destructured: {name} has {marks}");

    let saffron = Rgb(255, 153, 51);
    println!("Saffron is rgb({}, {}, {})", saffron.0, saffron.1, saffron.2);

    let _boss = Admin;
}
