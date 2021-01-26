fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Arr(u8, u8, u8);

fn Tuples(){
    let long_tuples = ((22, 44, 66, 88), (-22, -11, -1), 0);
    println!("long tuple: {:?}", long_tuples);

    //use a function reverse for reverse my next variable
    let pair = (3, false);
    println!("now pair is: {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    let arr = Arr(1,2,3);
    println!("My arr has: {:?}", arr)
}