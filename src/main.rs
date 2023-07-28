mod commands;
use crate::commands::Commands;
use clap::{ Arg, App };

fn main() {
    let commands = Commands {};
    let matches = App::new("My Program")
        .version("1.0")
        .author("Your Name Kolia")
        .about("Does awesome things")
        .arg(Arg::with_name("l").help("Sets the input file to use").index(1))
        .arg(Arg::with_name("a").help("Sets the input file to use").index(2))
        .get_matches();
    let ls = match matches.value_of("l") {
        Some(inp) => inp,
        None => "",
    };
    if ls == "" {
        let _ = commands.show_content();
    } else if ls == "l" {
        let _ = commands.show_more_content();
    } else {
        println!("Command not found");
    }
}
