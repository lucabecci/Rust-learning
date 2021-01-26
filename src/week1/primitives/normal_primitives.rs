fn NormalPrimitives(){
    let lgc: bool = false;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer: i32 = 7;   // `i32`

    let mut inferred_type: i64 = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable: i32 = 32;
    mutable = 322;

    // Variables can be overwritten with shadowing.
    let mutable = true;

}

fn LiteralsAndOperators(){
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}