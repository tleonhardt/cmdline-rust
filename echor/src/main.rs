use clap::Parser;

/// Rust echo
#[derive(Parser, Debug)]
#[clap(about, version, author="Ken Youens-Clark <kyclark@gmail.com>")]
struct Args {
    /// Input text
    #[clap(required=true)]
    text: Vec<String>,

    /// Do not print newline
    #[clap(short)]
    newline_omit: bool,
}

fn main() {
    let args = Args::parse();

    print!("{}{}", args.text.join(" "), if args.newline_omit { "" } else { "\n" });
}
