fn shadow_with_scopes(){
    let x = 1;

    println!("Value of x: {x}");
    
    let x = x + 1;

    println!("Shadow value of x: {x}");

    {
        let x = x + 2;

         println!("Shadow value of x in scope: {x}");
    }

    println!("Shadow value of x outside of scope: {x}");
}

fn shadow_change_type(){
    let spaces = "      ";

    println!("Spaces: {spaces}.");

    let spaces = spaces.len();

    println!("Spaces after shadow: {spaces}");

    //However this won't work as we are not shadowning
    //
    //let mut spaces = "      ";
    //
    //spaces = spaces.len();
    //
    //this will throw a compiler error as we are reassigning the value

}

fn assignment(){
    let t = true; // the type is inferred

    let f: bool = false; // with explicit type annotation
}

fn tuples(){
    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Value of y from tuple {y}");

    let val = tup.0;
    let val = val.to_string(); 
    println!("Value of first value in tuple {val}");

}

fn arrays(){
    let x = [1,2,3,4,5];

    let y : [i32; 5] = [1,2,3,4,5];

    let z = [3;5];
}

fn main() {
    shadow_with_scopes();
    shadow_change_type();
    assignment();
    tuples();
    arrays();
}
