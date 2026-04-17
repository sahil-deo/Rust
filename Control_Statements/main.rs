use std::io;

fn main(){
    // If Statements
    let mut input:String = String::new();

    println!("Enter Age: ");
    io::stdin().read_line(&mut input).expect("Failed");
    
    let age: i32 = input.trim().parse::<i32>().unwrap();
    input.clear();
    if age >= 18 {
        println!("Adult");
    } 
    else if age >= 11 {
        println!("Teen");
    }
    else{
        println!("Child");
    }

    // let var = if condition {a} else {b};
    let can_drive: bool = if age >= 18 {true} else {false};

    if can_drive {
        println!("Can Drive");
    }else {
        println!("Cannot Drive");
    }

    // Loops

    let mut i: u32 = 0;
    
    // this is an unconditional loop which runs till explicitly stopped using the break keyword.
    // Loop returns the value specified after the break statement.
    let result = loop {
        println!("Hello World");
        i+=1;
        if i > 3 {
            break i; // i will be returned.
        }
    };

    println!("Result is {}", result);

    // Loop Labels
    // For nested loops

    let mut count = 0;
    // loop label
    'outer: loop {
        println!("Count: {}", count);
        let mut remaining = 10;
        loop {
            count+=1;
            if count == 2 {
                break 'outer; // breaks the outer loop directly
            }
            remaining-=1;
            println!("Remaining: {}", remaining);
        }
    }

    // while loop 
    let mut number = 3;
    while number != 0{
        println!("{number}");
        number-=1;
    }

    // For loop
    let a = [1,2,3,5];
    for el in a {
        println!("{el}");
    }


}