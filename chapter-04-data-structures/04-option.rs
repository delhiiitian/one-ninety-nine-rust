// Chapter 4, Lesson 4: Option, a value that might be missing
// Run: cargo run --example ch04-04

struct Contact {
    name: String,
    phone: Option<String>, // some contacts have no phone number
}

fn find_contact<'a>(contacts: &'a [Contact], name: &str) -> Option<&'a Contact> {
    for c in contacts {
        if c.name == name {
            return Some(c);
        }
    }
    None
}

fn main() {
    let contacts = vec![
        Contact { name: "Asha".into(), phone: Some("98100 00001".into()) },
        Contact { name: "Dev".into(), phone: None },
    ];

    // match handles both cases explicitly.
    for name in ["Asha", "Dev", "Zoya"] {
        match find_contact(&contacts, name) {
            Some(contact) => match &contact.phone {
                Some(phone) => println!("{name}: {phone}"),
                None => println!("{name}: no phone saved"),
            },
            None => println!("{name}: not in contacts"),
        }
    }

    // Handy Option methods
    let dev = find_contact(&contacts, "Dev").unwrap(); // we know Dev exists
    let phone = dev.phone.as_deref().unwrap_or("unknown");
    println!("Dev's phone: {phone}");

    // map transforms the value inside Some, and leaves None alone.
    let name_length: Option<usize> = find_contact(&contacts, "Asha").map(|c| c.name.len());
    println!("Asha's name length: {name_length:?}");

    // is_some / is_none
    println!("Is Zoya saved? {}", find_contact(&contacts, "Zoya").is_some());

    // Option is how Rust avoids null. You cannot use an Option<String>
    // as a String by accident; the compiler makes you handle None.
    let middle_name: Option<&str> = None;
    println!("Middle name: {}", middle_name.unwrap_or("-"));
}
