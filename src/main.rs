mod cli;
mod util;
// use tracing::{span,Level};
use clap::Parser;
use std::process;
use util::find;
use util::get_regex;
use cli::check_cli;
use colored::*;
use cli::Cli;

fn main() {
    // let span = span!(Level::TRACE,"my span");
    // let span = span.enter();
    let mut cli = Cli::parse();
    check_cli(&mut cli);
    let regex = get_regex(cli.regex);
    for path in cli.path{

        if cli.verbose{
            println!("Now search in path : {}",path);
        }

        match find(path,&regex) {
            Ok(mut matches) => {
                if matches.is_empty(){
                    println!("No match.");
                } else {
                    println!("find the following mathces: ");
                    if cli.asc{
                        matches.sort();
                    }else if cli.desc{
                        matches.sort_by(|a, b| b.cmp(a));
                    }
                    for path in matches {
                        if let Some(index) = path.rfind('/'){
                            let (p,f) = path.split_at(index+1);
                            println!("{}{}",p.blue(),f.green());
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        }
    }
}


