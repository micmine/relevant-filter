use clap::Parser;

/// A filter for logana report 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// This is the reference to a branch or commit that has to be compared 
    #[clap(short, long)]
    pub since: String
}
