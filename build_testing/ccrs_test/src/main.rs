
extern "C" {
    pub fn xd();
}

fn main() {
    println!("C... ");
    unsafe { xd(); }
}
