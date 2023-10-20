use clap::Parser;
use cuid2::CuidConstructor;

#[derive(Parser, Debug)]
#[command(author = "Jordan Bravo, jordan.bravo.cc", version="0.1.0", )]

/// ID Gen (idgen) is a command line tool to generate collision resistant user IDs (CUIDs), which are a more modern version of UUIDs.  For more information on the topic, see https://github.com/paralleldrive/cuid2
///
/// idgen accepts two command line arguments:
///
/// 1. Number of characters each id should contain, 1 - 100.
///
/// 2. Number of ids to generate, 1 - 255.
struct Args {
    #[arg(short, long, default_value_t = 1)]
    length: u16,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    // Store the start time
    let start_time = std::time::Instant::now();
    // Store the command line arguments passed
    let args = Args::parse();
    // Handle the edge cases of the length argument
    let length: u16;
    if args.length > 100 {
        length = 100;
    } else {
        length = args.length;
    }
    // Create CUID2 constructor
    let constructor = CuidConstructor::new().with_length(length);
    // Handle the edge cases of the count argument
    let count: u8;
    if args.count > 255 {
        count = 255;
    } else if args.count == 0 {
        count = 1;
    } else {
        count = args.count;
    }
    if count < 2 {
        println!("Generating {} CUID with length {}: \n", count, length);
    } else {
        println!("Generating {} CUIDs with length {}: \n", count, length);
    }
    // Generate and print the IDs
    for _ in 0..count {
        let id = constructor.create_id();
        println!("{}", id)
    }
    println!("");
    eprintln!("Completed in {:?} seconds", start_time.elapsed());
}
