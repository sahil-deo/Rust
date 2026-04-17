fn main(){


    let x: i32 = 5;
    let y: &i32 = &x; // immutable reference. 

    // Gives error because y is an immutable reference to x. 
    // *y+=1; 

    println!("X: {} Y: {}", x, y);

    let mut x: i32  = 5;  // mutable owner. 
    let y: &mut i32 = &mut x; // mutatble reference. 

    *y+=1;

    // gives error - a value can only have 1 mutable reference or n immutable refernce.
    // println!("X {} Y {}", x, y);
    // since 1 mutable reference is created and given to y, using in println! is not possible as it creates a immutable reference.

}