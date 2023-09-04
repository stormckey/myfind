mod cli;
mod util;
use clap::Parser;
use std::process;
use util::find;
use regex::Regex;
use cli::check_cli;
use cli::Cli;
use colored::*;
use tracing::{span, Level, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let scope = span!(Level::DEBUG, "main");
    let _enter = scope.enter();
    let mut cli = Cli::parse();
    check_cli(&mut cli);
    let regex = match Regex::new(cli.regex.join("|").as_str()){
        Ok(re) => re,
        Err(err) => {
            eprintln!("illegal regex: {}", err);
            process::exit(1);
        }
    };
    info!("The regex now is {}", regex.as_str());
    for path in cli.path{

        if cli.verbose{
            println!("Now search in path : {}",path);
        }
        info!("The path to be searched now is {}",path);
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