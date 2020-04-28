use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let x = vec![1, 2, 3, 4];
    
    let mut v = Vec::new();
    
    v.push(5);
    v.push(124);
    
    for i in &v {
        println!("v[{}] l={}, cap={}", i, v.len(), v.capacity());
    }
    
    v.clear();
    
    match v.pop() {
        Some(x) => println!("last {}", x),
        None => println!("No last.")
    }
    
    let mut t: Option<i64> = None;
    
    t = Some(4);
    t = None;
    
    println!("t = {}", match t {
        Some(x) => x.to_string(),
        None => "nyet!".to_string(),
    });
    
    // vector = same types
    // but i can use enum because its the same type
    // enum is therefore an union type
    
    #[derive(Debug)]
    enum ContainerValue {
        Float(f64),
        Int(i64),
        Text(String),
    }
    
    let container_vector = vec![
        ContainerValue::Int(142),
        ContainerValue::Text("kys".to_string())
    ];
    
    println!("{:?}", container_vector);
    
    // hashmaps
    let mut hm = HashMap::new();
    hm.insert("yes".to_string(), 3);
    hm.insert("kys".to_string(), 44);
    
    hm.remove(&"kys".to_string());
    
    for (k, v) in &hm {
        println!("{} -> {}", k, v);
    }
    
    // panik if none
    // let value_yes = hm.get(&"yes".to_string()).unwrap();
    
    let get_opt = hm.get(&"yes".to_string());
    
    // match the value and retrieve it in match
    match get_opt {
        Some(x) => println!("yes => {}", x),
        _ => println!("not found")
    }
    
    // if let binding
    if let Some(x) = get_opt {
        println!("GOT X : {}", x);
    } else {
        println!("nope havent got");
    }
    
    let a_char = '😂';
    println!("{}", a_char);
    
    if let Ok(f) = File::open("file1.txt") {
        println!("yes ok");
    } else {
        println!("nein")
    }
    
    
    
    Ok(())
}
