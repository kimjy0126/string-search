use std::env;                   // for reading arguments
use std::fs::File;              // for file io
use std::io;                    // for error handling in file io
use std::io::prelude::*;        // for file io
use string_search::*;

fn print_usage(args: &str) {
    println!("usage: {} <needle> <haystack filename>", args);
}

fn make_haystack_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let mut f = File::open(filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut haystack: Vec<String> = vec![];
    for line in contents.lines() {
        haystack.push(line.to_string());
    }

    Ok(haystack)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        return;
    }

    let haystack: Vec<String> = match make_haystack_from_file(&args[2]) {
        Ok(v) => v,
        Err(_) => {
            println!("Error: Failed to get input from file");
            return;
        }
    };
    let needle: String = args[1].clone();

    let searcher = Searcher::new()
                        .replace_initial(true)
                        .ignore_whitespace(true)
                        .case_sensitive(false);
    
    let result = searcher.search(&needle, &haystack);
    for res in result.iter() {
        println!("{}", res);
    }
}