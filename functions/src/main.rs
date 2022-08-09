fn returns_a_value() -> i32{
    5
}

fn has_param(x: i32){
    println!("x: {x}");
}

fn main() {
    has_param(5);
    let value = returns_a_value();
    println!("Value from func {value}");


    // let x = 5; This is a statement as shown by the ;
    // let x = {
    //     5 //This is a expression as it has no ;
    // }
}
