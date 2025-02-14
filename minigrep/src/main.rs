use std::env;
use std::fs;
use std::process;
use std::error::Error;

// in main.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Args::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Args) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}

struct Args {
    query: String,
    file_path: String,
}
// Config 中存储的并不是 &str 这样的引用类型，而是一个 String 字符串，也就是 Config 并没有去借用外部的字符串，而是拥有内部字符串的所有权
// 这就意味着，当 Config 被 Drop 掉的时候，内部的字符串也会被释放掉，而不会影响外部的字符串

impl Args {
    fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Args { query, file_path })   
    }
}