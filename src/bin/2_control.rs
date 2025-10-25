fn main() {
    // 1. if statement
    if if_larger(5, 2) {
        println!("of course, {} is larger than {}", 5, 2);
    }

    println!("{}", return_larger(5, 24));

    // 2. loop
    loop_count("The message", 10);

    // 3. nested loop
    nested_loop(5);
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

fn nested_loop(jump_count: u32) -> u32 {
    // this function to show how to break nested loop by label
    let mut count = 0;
    'outer: for _i in 0..10 {
        for _j in 0..10 {
            count += 1;
            if count > jump_count {
                // specify break labeled-loop "outer"
                break 'outer;
            }
        }
        count += 1000;
    }
    count
}