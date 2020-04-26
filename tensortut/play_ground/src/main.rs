fn main() {
    println!("Hello, nibba.");

    let x: u32 = 5;
    println!("x = {}", x);

    // tuples
    let v: (i64, i64) = (42, 33);
    let (_, y) = v;
    println!("y = {}", y);

    // arrays
    let a = [1, 2, 3, 4];
    println!("a[2] = {:?}", a[2]);


}
