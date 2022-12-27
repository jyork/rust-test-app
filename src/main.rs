
fn main() {
    println!("Hello, world");

    var_and_print();

    test_implicit_return();    
}

fn var_and_print() {
    // Variables can be type annotated.
    let logical: bool = true;

    println!("Bool: {}", logical);

    // Unused variables.
    // By default Rust flags unused varaibles as errors, but you can override this by
    // starting the variable name with an underscore.
    let _unused_var = false;

    let flt_pi: f64 = 3.14159;  // Regular annotation
    let i32_meaning_of_life   = 42i32; // Suffix annotation

    println!("Numbers: float {}, i32 {}", flt_pi, i32_meaning_of_life);

    println!("Formatted to: two decimal points {:2}", flt_pi);
    println!("Formatted to: zero filled to 8 digits {0:08.2}", flt_pi);

    println!("Formatted to: hex {:X}, binary {0:b}", i32_meaning_of_life);

    // Or a default will be used.
    let _default_float   = 3.0; // `f64`
    let _default_integer = 7;   // `i32`

    println!("Numbers (default type): f64 {}, i32 {}", _default_float, _default_integer);

    // Mutable means value of a variable can change.
    let mut _mut_i64 = 1234i64;
    // A type can also be inferred from context 
    let mut _mut_inferred_i64= 12; // Type i64 is inferred from another line
    _mut_inferred_i64 = 4294967296i64;
    println!("Mutable vars: inferred type {}, i64 {}", _mut_inferred_i64, _mut_i64);
    _mut_i64 += 1;
    _mut_inferred_i64 += 1;
    println!("Incremented: inferred type {}, i64 {}", _mut_inferred_i64, _mut_i64);

    // Error! The type of a variable can't be changed.
    //_mutable = true;
    
    // Variables can be overwritten with shadowing.
    let _mut_inferred_i64 = true;
    println!("Overwitten shadowed to bool: {}", _mut_inferred_i64)
}

// Implicit return
fn test_implicit_return()
{
    println!("Is odd(5) {}", is_odd(5))
}

// Note that semi-colon must be omitted.
fn is_odd(x: i64) -> bool
{
    x % 2 != 0
}
