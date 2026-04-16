//-----------
// Ownership
//-----------

// Rules of Ownership
// 1. Each value in Rust has a variable that is its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


fn main(){
    
    let s1 = String::from("RUST");
    
    // passing only the reference of s1 to the calculate_length function, so the owner still remains s1
    let len = calculate_length_1(&s1); // passing by reference by using &s1 not s1 (pass ownership)
    println!("Length of '{}' is {}", s1, len); 


    // passing the ownership of the value "RUST" from s1 to s2.
    let s2 = s1;

    // invoking s1 results in an error because, s1 no longer holdes a value, as its value has been passed onto s2.
    // println!("S1: {}", s1); 
    println!("S2: {}", s2); // This works as s2 is assigned the owner of the value "RUST".


    // passing the ownership of the value "RUST" from s2 to the function calculate_length_2
    let len = calculate_length_2(s2); 
    // println!("Length of {} is {}", s2, len); // Invoking s2 will now result in an error because it is not an owner of a value. 
    println!("Length is {}", len);

    let s1 = String::from("RUST"); // new String value is given to s1
    println!("s1: {}", s1);
    { 
        let s2 = s1; // Passed ownership to s2, so s1 is empty.
        println!("s2: {}", s2);
    }
    // Gives error since s1 is empty.
    // println!("s1: {}", s1);


    // declaring a String reference outside the scope.
    let s1: &String;

    {
        // declaring a String variable with value inside the scope.
        let s2: String = String::from("RUST");
        // assigning reference to s2 to s1 in the scope.
        s1 = &s2;
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        // s2 deleted at the end of the scope, so did its values, and all references.
    }

    // Gives Error - s2's reference deleted at the end of the scope. 
    // a value cannot remain without an owner. 
    // println!("s1: {}", s1);

}




fn calculate_length_1(s: &String) -> usize {
    // only accepts the reference of a String by using &String.
    s.len()
}

fn calculate_length_2(s: String) -> usize {
    // accepts the ownership of a String value
    s.len()
    // the value of the String s is deleted here, because s is going out of scope. 
}





