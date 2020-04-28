fn main() {
    let n = 5;
    
    if n < 5 {
        println!("mensie")
    } else {
        println!("vacsie alebo rovne")
    }
    
    let a = vec![1, 2, 3];
    for x in a {
        println!("x: {}", x);
    }
    
    
    let mut v: Vec<i64> = vec![];
    
    for i in 0..=4 {
        v.push(i);
    }
    
    println!("v: {:?}", v);
    
    let x = 5;
    let s = match x { // pattern matching
        14..=50 => "iterated",
        1 => "ONE",
        2 => "TWO",
        4 | 3 => "4 or 3",
        _ => "DEFAULT",
    };
    
    println!("s = {}", s);
    
    
    let pair = (0, 5);
    
    let non_zero = match pair {
        (0, y) => y,
        (x, 0) => x,
        _ => 0,
    };
    
    println!("nz = {}", non_zero);
    
    // stops at first match !!!
    match pair {
        (x, _) if x == 0 => println!("X is 0"),
        (_, y) if y == 0 => println!("Y is 0"),
        _ => println!("none"),
    }
    
    // ownership binding to variable inside match clause
    let p = 16;
    let n = match p {
        n @ 1..=12 => n, // use @ if we dont have ownership
        n @ 13..=19 => n,
        _ => 0
    };
    
    println!("source p = {}", p);
    println!("matched n = {}", n);
}
