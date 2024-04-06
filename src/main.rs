use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    first_path: std::path::PathBuf,
    second_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!(
        "First file: {}\nSecond file: {}",
        args.first_path.display(),
        args.second_path.display()
    )
}
