use std::io;

fn main(){
    let mut input = String::new();

    println!("Enter Height:");
    io::stdin().read_line(&mut input).expect("Failed");
    let h = input.trim().parse::<f64>().unwrap();

    input.clear();

    println!("Enter Weight:");
    io::stdin().read_line(&mut input).expect("Failed");
    let w = input.trim().parse::<f64>().unwrap();

    println!("BMI:{:.2}", {
        let h_squared: f64 = h*h;
        let bmi:f64 = w/h_squared;
        bmi
    });
}