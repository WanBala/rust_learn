fn main() {
    // 1. if statement
    if if_larger(5, 2) {
        println!("of course, {} is larger than {}", 5, 2);
    }

    println!("{}", return_larger(5, 24));

    // 2. loop
    loop_count("The message", 10);
}

fn if_larger(x: u32, compare: u32) -> bool {
    // x > compare
    if x > compare { true } else { false }
}

fn return_larger(x: u32, compare: u32) -> u32 {
    if x > compare {
        return x;
    } else if x < compare {  // use "else if" to cascade another condition
        return compare;
    }
    x
}

fn loop_count(msg: &str, count: u8) {
    let mut counter = 0;
    loop {
        println!("{msg} shows {count} times");
        counter += 1;
        if counter == count {
            break
        }
    }
}