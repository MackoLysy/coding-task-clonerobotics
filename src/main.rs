use clap::Parser;

mod args;

fn main() {
    let parser = args::ArgParser::parse();
    println!("Hello, world! {}", parser.get_host());
}
