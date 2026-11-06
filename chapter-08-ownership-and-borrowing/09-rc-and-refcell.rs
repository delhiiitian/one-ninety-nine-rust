// Chapter 8, Lesson 9: Shared ownership with Rc, and RefCell
// Run: cargo run --example ch08-09

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Playlist {
    name: String,
    songs: RefCell<Vec<String>>, // can be changed even through a shared Rc
}

fn main() {
    // Rc = "reference counted": several owners share ONE value. It is freed
    // when the last owner goes away. Rc::clone copies the pointer, not the data.
    let road_trip = Rc::new(Playlist { name: "Road Trip".into(), songs: RefCell::new(vec!["Kesariya".into()]) });
    println!("Owners: {}", Rc::strong_count(&road_trip));

    let asha_view = Rc::clone(&road_trip);
    let dev_view = Rc::clone(&road_trip);
    println!("Owners after sharing: {}", Rc::strong_count(&road_trip));

    // Rc gives only shared (&) access. RefCell moves the borrowing check
    // from compile time to run time, so we can still change the songs.
    asha_view.songs.borrow_mut().push("Naatu Naatu".into());
    dev_view.songs.borrow_mut().push("Jai Ho".into());

    println!("{} now has: {:?}", road_trip.name, road_trip.songs.borrow());

    {
        let _extra = Rc::clone(&road_trip);
        println!("Owners inside block: {}", Rc::strong_count(&road_trip));
    }
    println!("Owners after block: {}", Rc::strong_count(&road_trip));

    // RefCell still enforces the rules, at run time: two mutable borrows at
    // once is a panic. try_borrow_mut reports it instead of panicking.
    let first = road_trip.songs.borrow_mut();
    match road_trip.songs.try_borrow_mut() {
        Ok(_) => println!("unexpected second borrow"),
        Err(_) => println!("Second mutable borrow refused while the first is active"),
    }
    drop(first);
    println!("After dropping the first borrow: {} songs", road_trip.songs.borrow().len());

    // A shared counter with Rc<RefCell<T>>: the classic combination.
    let likes = Rc::new(RefCell::new(0));
    let buttons: Vec<Rc<RefCell<i32>>> = (0..3).map(|_| Rc::clone(&likes)).collect();
    for b in &buttons {
        *b.borrow_mut() += 1;
    }
    println!("Total likes from 3 buttons: {}", likes.borrow());
}
