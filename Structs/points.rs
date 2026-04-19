struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn calculate_distance(&self, p2: &Point) -> f64{
        let x = (p2.x-self.x).pow(2);
        let y = (p2.y-self.y).pow(2);
        let d: f64 = (x+y) as f64;
        d.sqrt()
    }
    fn print(&self) {
        println!("({},{})", self.x, self.y);
    }
}

fn main(){

    let p1: Point = Point{x:4,y:2};
    let p2: Point = Point{x:4,y:4};

    println!("Point 1");
    p1.print();

    println!("Point 2");
    p2.print();

    println!("Distance: {:.2}", p1.calculate_distance(&p2));
}