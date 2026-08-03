fn main() {
    basic_operations();
    print_separator();

    inner_scope();
    print_separator();

    tuples();
    print_separator();

    arrays();
    print_separator();

    control_flow();
    print_separator();
}

fn control_flow() {
    println!("control_flow");
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of number if {num}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");

    println!("** loop labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("** while loops");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!("** for-loop with reverse order");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn arrays() {
    println!("arrays");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3, 5]; // array of 2 elements 3 and 5
    let c = [3; 5]; // array of 5 elements each being 3
    let d: [i32; 0] = [];

    print_array(&a);
    print_array(&b);
    print_array(&c);
    print_array(&d);
}

fn print_array(arr: &[i32]) {
    if arr.is_empty() {
        println!("empty array");
        return;
    }

    let last_index = arr.len() - 1;

    for (i, element) in arr.iter().enumerate() {
        if i == last_index {
            print!("{element}");
        } else {
            print!("{element} -> ");
        }
    }
    println!();
}

fn tuples() {
    println!("tuples");
    let tup = (500, 6.4, 1, "kek", 'Ъ');

    let i = tup.0;
    println!("{i}");

    let (first, second, third, forth, fifth) = tup;
    println!("{first}, {second}, {third}, {forth}, {fifth}");
}

fn basic_operations() {
    println!("basic_operations");
    let mut x = 5;
    print_value(x);

    x = 6;
    print_value(x);
}

fn print_value(x: i32) {
    println!("value is: {x}");
}

fn inner_scope() {
    println!("inner_scope");
    let x = 5;
    println!("The initial value of x is {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn print_separator() {
    println!("*******");
}
