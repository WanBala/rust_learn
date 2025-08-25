fn main() {
    //===== 1. Variables =====
    let mut x = 5;
    println!("the value x: {x}");
    x = 6;
    println!("the value x: {x}");

    const JULY: u8 = 7;
    println!("the July is : {JULY}");

    //===== 2. Shadow =====
    let spaces  = "Hello World";
    let spaces = spaces.len();
    println!("Length of 'Hello World' : {spaces}");

    //===== 3. Data Type ====
    // Note. prefix "_" of the variable is to disable compiler warning for unuse. 
    let _var_u8: u8 = 255;
    let _var_u8 = 255u8;

    let _var_usize: usize = 65535;
    let _var_usize = 65535usize;

    let _var_i32: i32 = 255;
    let _var_i32 = 255i32;
    let _var_i32 = 255;  // i32 as default for integer

    let _var_f64: f64 = 3.14159;
    let _var_f64 = 3.14159f64;
    let _var_f64 = 3.14159; // f64 as default for floating point

    let _var_f64 = 3.14159 - 3.;
    let _var_f64 = 3.14159 - (3 as f64); // cast 3 to f64

    let _var_bool = true;
    let _var_bool: bool = true;

    let _var_char = '😊'; // In Rust, char is represented as a UTF-32 code point (4 bytes).
    let _var_char: char = '😊';
    println!("{_var_char}");
}