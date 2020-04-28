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

fn main() {
    let u = Direction::Up(Point{x: 4, y: 5});
    let k = u.match_direction();
    let destr = k.destruct();
    
    println!("u = {:?}", u);
    println!("k = {:?}", k);
    println!("destruct = {:?}", destr);
}
