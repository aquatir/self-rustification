fn main() {
    let separator = String::from("****");

    println!("{separator}");
    println!("simple_ownership");
    simple_ownership();

    println!("{separator}");
    println!("pointers");
    pointers();

    println!("{separator}");
    println!("slices");
    slices();
}

fn slices() {}

fn pointers() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{s1}' is {len}.");

    // s is mutable
    let mut s = String::from("hello");

    change(&mut s);
    println!("new s: {s}");

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // this second assignment won't work because only one mutable reference (&mut) can exist in scope

    // having multiple immutable references is fine though
    // we can not mutate s while r1 and r2 are in scope here
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("s: {s}, r1: {r1}, r2: {r2}");

    // the push_str line technically works, but because r1/r2 are used in a println
    // this push is not valid (both mutable and immutable references exist)
    // however, you can push, and then print JUST s - because you don't reference r1/r2
    s.push_str(", world");
    // println!("s: {s}, r1: {r1}, r2: {r2}");
    println!("s: {s}");

    let r3 = &mut s; // no problem
    r3.push_str(", world");
    println!("{r3}");
}

// an accepted reference is also mutable: makes it clear we can change it
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn simple_ownership() {
    let mut s1 = String::from("hello");
    println!("s: {s1}");

    s1.push_str(", world");
    println!("new s: {s1}");

    let s2 = s1; // moving the ownership from s1 to s2
    println!("s2: {s2}");

    // println!("s: {s1}"); // invalid because ownership of string s1 moved to s2

    let s3 = s2.clone(); // making an explicit copy of s2 into s3, now they refer different allocation on heap
    println!("s3: {s3}");

    takes_ownership(s3);
    // println!("s3: {s3}"); // can't use it because a function above has taken an ownership

    let (s4, len) = calculate_length(s2);
    println!("The length of '{s4}' is {len}.");
}

fn takes_ownership(str: String) {
    println!("{str}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
