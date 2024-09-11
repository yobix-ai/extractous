use std::env;
use std::process;
use std::time::Instant;

use extractous::Extractor;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path is provided
    if args.len() != 2 {
        eprintln!("\nUsage: cli <file_path>\n");
        process::exit(1);
    }

    // Get the file path from the arguments
    let file_path = &args[1];

    // Call the parse function and measure the time taken
    let start_time = Instant::now();
    let content = Extractor::new()
        .set_extract_string_max_length(1000000)
        .extract_file_to_string(&file_path);
    let call_duration = start_time.elapsed();

    match content {
        Ok(c) => {
            println!("{}", c)
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let total_duration = start_time.elapsed();

    println!("Time taken to parse: {:.4?}", call_duration);
    println!("Total time taken: {:.4?}", total_duration);
}
