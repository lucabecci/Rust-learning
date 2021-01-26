fn NormalPrints(){
    //first test
    println!("{} persons in the room", 15)

    //second test
    println!("{0}, this is {1}, she is my sister", "Carlos", "Elena")

    //third test
    println!(
        "{first} ||||||||||||| {second}",   
        first = "The first argument",
        second = "The second argument"
    )
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "James", "Bond");
}