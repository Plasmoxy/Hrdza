/*
Brainfucc by Plasmoxy 2020

> = increases memory pointer, or moves the pointer to the right 1 block.
< = decreases memory pointer, or moves the pointer to the left 1 block.
+ = increases value stored at the block pointed to by the memory pointer
- = decreases value stored at the block pointed to by the memory pointer
[ = like c while(cur_block_value != 0) loop.
] = if block currently pointed to's value is not zero, jump back to [
, = like c getchar(). input 1 character.
. = like c putchar(). print 1 character to the console

*/

#[derive(PartialEq)]
enum EntryType {
    None,
    Number,
    String,
}

fn main() {
    // get source
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("You must specify a source file.");
        return;
    }
    
    let source_path = args.get(1).unwrap();
    let source = match std::fs::read_to_string(source_path) {
        Ok(x) => x,
        _ => {
            println!("Error reading specified file.");
            return;
        }
    };
    
    let chars: Vec<char> = source.chars().collect();
    let s_len = chars.len();
    
    // the interpreter
    // 32 bit integer cells
    // 255 nested loops possible
    
    let mut block: [i32; 30000] = [0; 30000];
    let mut ip: usize = 0; // instruction pointer in source file
    let mut bp: usize = 0; // block pointer inside block (current cell index)
    let mut loops: [usize; 255] = [0; 255];
    let mut lp: usize = 0; // current loop is lp-1, lp==0 means no current loop
    
    // flags
    let mut current_entry = EntryType::None;
    let mut entry_str = String::new();
    
    while ip < s_len {
        let instruction = chars.get(ip).unwrap();
        
        // execute instructions
        match instruction {
            // basic Brainfuck instruction set
            
            '>' => bp += 1,
            '<' => bp -= 1,
            '+' => block[bp] += 1,
            '-' => block[bp] -= 1,
            
            '[' => {
                loops[lp] = ip;
                lp += 1;
            },
            
            ']' => {
                if lp == 0 {
                    println!("Error: tried to close loop while not inside any.");
                    return;
                }
                if block[bp] != 0 {
                    ip = loops[lp - 1];
                } else {
                    lp -= 1;
                }
            },
            
            '.' => {
                print!("{}", std::char::from_u32(block[bp] as u32).unwrap_or('$'))
            },
            
            // custom macros / extensions (ignored by normal bf)
            '#' => display_block(&block, 0, 10, bp),
            
            // enter number-entering mode, mode is exited when normal brainfuck instruction is reached
            // '_' => { current_entry = EntryType::Number; },
            
            '\"' => {
                
            },
            
            // non instruction characters
            x => {
                if current_entry != EntryType::None {
                    entry_str.push(*x);
                }
            },
        }
        
        ip += 1;
    }
}


// note, all values except block are copied, not moved
// block is referenced
fn display_block(block: &[i32; 30000], from: usize, to: usize, bp: usize) {
    let mut ss = String::from(format!("# {}=[", from));
    for i in from..=to {
        ss.push_str(& if i == bp {
            format!("({})", block[i])
        } else {
            format!("{}", block[i])
        });
        if i != to {
            ss.push_str(", ")
        }
    }
    ss.push_str(&format!("]={}", to));
    println!("{}", ss);
}