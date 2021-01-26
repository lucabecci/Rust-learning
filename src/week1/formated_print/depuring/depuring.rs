#[derive(Debug)] //I added this for print structures
struct UnPrintable(i32);

#[derive(Debug)]
struct Deep(UnPrintable);

fn Depuring(){
    // `Structure` is printable!
    println!("Now {:?} will print!", UnPrintable(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(UnPrintable(733)));
}