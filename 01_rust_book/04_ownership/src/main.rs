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

fn slices() {
    let mut str = String::from("hello, world");
    let first_space = first_word(&str);
    println!("first word of string is '{}'", &str[0..first_space]);

    let s = String::from("hello");

    // diff ways to use slices
    let slice = &s[0..2];
    let slice = &s[..2];
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // you can drop both so these 2 calls are equal
    let slice = &s[0..len];
    let slice = &s[..];

    let substring = first_word_slice(&str);
    println!("first work of a string sliced is {substring}");

    // this won't work anymore, because &str borrows a string as immutable,
    // but clear is applied on mutalbe only
    // works without a println (before usage)

    // str.clear();
    // println!("first work of a string sliced is {substring}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

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
