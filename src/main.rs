use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The file to summarize
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.file)
        .expect(&format!("Couldn't read file {:?}", &args.file)[..]);
    println!("{}", &content);
}

