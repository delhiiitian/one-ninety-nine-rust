// Chapter 1, Lesson 7: Tuples and arrays
// Run: cargo run --example ch01-07

fn main() {
    // A tuple groups a few values of possibly different types.
    let student: (&str, u8, f32) = ("Riya", 21, 8.7);
    println!("Name: {}, age: {}, CGPA: {}", student.0, student.1, student.2);

    // Destructuring unpacks a tuple into separate variables.
    let (name, age, cgpa) = student;
    println!("{name} is {age} with a CGPA of {cgpa}");

    // An array holds a fixed number of values of the SAME type.
    let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    println!("First day: {}, last day: {}", weekdays[0], weekdays[4]);
    println!("Number of weekdays: {}", weekdays.len());

    // [value; count] makes an array filled with the same value.
    let mut scores = [0; 5];
    scores[2] = 95;
    println!("Scores: {scores:?}"); // {:?} prints the whole array

    // Loop over an array.
    let temps = [31.5, 33.0, 29.8, 35.2];
    let mut hottest = temps[0];
    for t in temps {
        if t > hottest {
            hottest = t;
        }
    }
    println!("Hottest day this week: {hottest} C");

    // Reading past the end is caught: weekdays[10] would stop the program
    // with "index out of bounds" instead of reading random memory.
    match weekdays.get(10) {
        Some(day) => println!("Day 10 is {day}"),
        None => println!("There is no day at index 10"),
    }
}
