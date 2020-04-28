use std::mem;
use std::fmt;

fn main() {
    println!("Hello, nibba.");

    {
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
    }

    {
        // strings ...
        let s: &str = "Stringslice";
        let sobj: std::string::String = String::from("Stringobj");
        let s_slice = &s[1..4];
        println!("{} {} {}", s, sobj, s_slice);

        let h = "Hello ".to_string();
        let w = "World".to_string();
        let conc = h + &w;
        println!("{}", conc);
    }

    {
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
        println!("borrowed: {}", ow_b);
    }
    
    {
        #[derive(Debug)]
        struct Object {
            width: u64,
            height: u64,
        }
        
        // impl creates a namespace
        impl Object {
            // const in namespace
            const FOUR: u64 = 4;
            
            // method
            fn area(&self) -> u64 {
                self.width * self.height
            }
            
            // doesnt have &self, therefore its static method
            // Object is also a namespace!
            fn new(width: u64, height: u64) -> Object {
                Object { width, height }
            }
        }
        
        impl Object {
            const NOVA_PREMENNA_XD: u64 = 69;
        }
        
        impl fmt::Display for Object {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "obj {} x {}", self.width, self.height)
            }
        }
    
        let o = Object {
            width: 64,
            height: 64,
        };
        
        println!("area: {}", o.area());
        println!("area 2: {}", Object::new(32, 33).area());
        
        println!("FOUR {}", Object::FOUR);
        
        println!("debug obj: {:#?}", o); // debug
        println!("display obj: {}", o); // display
    }
}
