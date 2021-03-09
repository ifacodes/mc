use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Args {
    /// Get debug output
    #[structopt(short, long)]
    debug: bool,

    /// Get the interval below the note
    #[structopt(short, long)]
    below: bool,

    #[structopt(short, long, default_value = "C")]
    key: String,
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);
}
