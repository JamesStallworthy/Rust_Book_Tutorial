fn main() {
    let s = "Hello"; //Fixed size and is baked into the executable, cannot be mutable

    let mut s = String::from("Hello"); //Unknown size, is stored on the heap and can be mutable
                                       //because of this

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);


    //--------------
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

    //---------------
    
    //Bind x to 5, make a copy of the data in x and bind that to y. This is done on the stack as
    //its fixed known size
    let x = 5;
    let y = x;

    //------------
    let s1 = String::from("hello"); //S1 variable does not hold the value "hello", It is actually
                                    //holding a pointer, len and capicity. The pointer is pointing
                                    //to the string data stored on the heap
    let s2 = s1; //S2 here is creating a copy of the stack data of s1, So a copy of the pointer,
                 //lenght and capicity. This is still pointing to the same string data on the heap
                 //as it would be expensive to copy this.
    
    //As rust clears up memory when a variable goes out of scope, this could cause the "hello" data
    //to be cleared up on the stack twice, once for s1 and once for s2. So in this case s1 is
    //considered s1 no longer valid and cannot be used going forward.
    //println!("{s1}") would throw a compiler error
    //
    //Most languages would consider this a shallow copy as the actual data does not change. In rust
    //this would be considered a move as s1 is no longer valid

    //----------
    {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    //This is a deep copy and will copy the heap. This means that both s1 and s2 would be valid
    //beyond this point

    //---------
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    } //x goes out of scope, s is now invalid so the stack will be cleared but the heap was cleared
      //when the takes_ownership method went out of scope
    
    let s1 = String::from("hello");

    let s2 = take_ownership_and_giveback(s1); //ownership of the heap data is given to the function
                                              //and then returned afterwards
    println!("{s2}");

    references();
    slices();
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn take_ownership_and_giveback(some_string: String) -> String{ //Takes takes ownership of the
                                                               //some_string data on the heap and
                                                               //then returns that data back at the
                                                               //end
    println!("{some_string}");
    some_string
}

//=====================================
// References
// ====================================

fn references(){
   let mut s1 = String::from("hello");

   let slen = calculate_len(&s1); //We pass a reference to s1. Behind the scenes this is passing a
                                  //pointer to the stack data containing the information about the
                                  //heap data. As this is a reference to this it will be safe to
                                  //use aslong as the scope is still valid
                                  //This means the calculate_len does not take ownership of the s1
                                  //variable

   println!("{s1} has the lenght of {slen}");

   change(&mut s1); // We can pass a mutable reference that will allow us to modify the data inside
                    // of the method
                    // You can only have 1 mutable reference to a variable at a time

   println!("{s1}");
}

fn calculate_len(s: &String) -> usize { 
    s.len()
} //The reference s is dropped here, but not the actual data! As we only have a reference to the
  //data here we are unable to modify it. This is known as borrowing in rust

fn change(s: &mut String) {
    s.push_str(", world!");
}


fn slices(){
   let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; //This will be stored as a reference with a lenght of 5 starting a
                           //postion 6 of the s data on the heap
    
    let slice = &s[0..2]; //These are the same
    let slice = &s[..2];

    //This is much better than just remembering the index and the end index as now the compiler can
    //keep an eye on the s value, if it goes out of scope or is modified then we will get a
    //compiler error as we have references to it!
}
