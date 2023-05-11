use std::env;
use std::process;

pub struct Args {
    pub memory: i32,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} -m <memory in GiB>", args[0]);
        process::exit(1);
    }
    let memory_arg = match args[2].parse::<i32>() {
        Ok(memory) => memory,
        Err(err) => {
            eprintln!("Error parsing memory argument: {}", err);
            process::exit(1);
        }
    };
    Args { memory: memory_arg }
}
