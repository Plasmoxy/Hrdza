#![allow(dead_code)]

// enum Direction {
//     Up (u32),
//     Down {x: u32, y: u64},
//     Left,
//     Right,
// }

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

// basically a type, can have a value but dont have to
// enum with additional data
#[derive(Debug)]
enum Direction {
    Up    (Point),
    Down  (Point),
    Left  (Point),
    Right (Point),
}

#[derive(Debug)]
enum Keys {
    UpKey    (String),
    DownKey  (String),
    LeftKey  (String),
    RightKey (String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey("Presed w".to_string()),
            Direction::Down(_) => Keys::DownKey("Pressed s".to_string()),
            Direction::Left(_) => Keys::LeftKey("Pressed a".to_string()),
            Direction::Right(_) => Keys::RightKey("Pressed d".to_string()),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

enum Shape {
    Rectangle { width: f64, height: f64 },
    Square(f64),
    Circle(f64),
}

// polymorphism
impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle {width, height} => width*height,
            Shape::Square(ref a) => a*a,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}


fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None }
    else { Some(a/b) }
}

fn main() {
    let u = Direction::Up(Point{x: 4, y: 5});
    let k = u.match_direction();
    let destr = k.destruct();
    
    println!("u = {:?}", u);
    println!("k = {:?}", k);
    println!("destruct = {:?}", destr);
    
    // ref keyword
    
    let source = 4;
    let r1 = &source; // r1 is reference to source
    let ref r2 = source; // same
    println!("r1==r2 {}", r1==r2);
    
    let r = Shape::Rectangle {width: 10.0, height: 4.2};
    let c = Shape::Circle(30.2);
    
    println!("circ area = {}, rect area = {}", c.area(), r.area());
    
    match divide(1.0, 2.0) {
        Some(x) => println!("{:.2}", x),
        None => println!("Division by zero.")
    }
    
    
}
