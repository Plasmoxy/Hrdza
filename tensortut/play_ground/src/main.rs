use std::mem;

fn main() {
    println!("Hello, nibba.");

    let x: u32 = 5;
    println!("x = {}", x);

    // tuples
    let v: (i64, i64) = (42, 33);
    let (_, y) = v;
    println!("y = {}", y);

    // arrays
    let a = [1, 2, 3, 4, 2, 4, 5];
    println!("a = {:?}", a); // debug fmt
    println!("size in mem: {}", mem::size_of_val(&a));

    
    // reference to part of array
    let a_sliced = &a[2..5];
    println!("sliced a: {:?}", a_sliced);

    // strings ...
    let s: &str = "Stringslice";
    let sobj: std::string::String = String::from("Stringobj");
    let s_slice = &s[1..4];
    println!("{} {} {}", s, sobj, s_slice);

    let h = "Hello ".to_string();
    let w = "World".to_string();
    let conc = h + &w;
    println!("{}", conc);

    // unit value
    // let t = (); // empty tuple is like None type

    // ownership
    // piece of data can only have one owner at a time
    let ow_s = String::from("Ownership");
    let ow_y = ow_s;
    // println!("{}", ow_s); // nope! -> ow_s is no longer the owner
    // it has been moved!
    // note: primitive types are not moved but copied
    println!("{}", ow_y);

    // borrowing
    let ow_b = &ow_y; // borrow it
    println!("{}", ow_b);
}
