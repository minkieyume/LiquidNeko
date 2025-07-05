use std::*;
use std::io::Write;
use nekolisp::env::Env;
use nekolisp::*;

fn main() {
    let env = Env::default();
    loop {
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0{
            println!("\nByeNyan");
            break;
        }
        let resutls = rep_str(&mut input,env.clone());
        for result in resutls {
            println!("{}",result)
        }
	io::stdout().flush().unwrap();
    }
}
