#[derive(Debug)]
struct Person<'name> {
    name: &'name str,
    age: u8
}

fn PrettyPrint() {
    let name = "Luca";
    let age = 20;
    let luca = Person { name, age };

    // Pretty print: #
    println!("{:#?}", luca);
}
