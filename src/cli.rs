use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub path: Vec<String>,
    
    #[arg(short, long)]
    pub regex: Vec<String>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub asc: bool,

    #[arg(short, long, conflicts_with = "asc")]
    pub desc: bool,
}

pub fn check_cli(cli: &mut Cli) {
    if cli.path.is_empty(){
        cli.path = vec![".".to_string()];
        if cli.verbose{
            println!("Use . as default path");
        }
    }
    if cli.regex.is_empty(){
        cli.regex = vec![".".to_string()];
        if cli.verbose{
            println!("Use . as default regex");
        }
    }
}