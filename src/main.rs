use clap::Parser;
use std::path::Path;

/// Bulk rename all files recursively with a string substitution.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root directory
    #[arg(short, long)]
    root_dir: String,

    /// Existing pattern
    #[arg(short, long)]
    existing: String,

    /// New pattern
    #[arg(short, long)]
    new_pattern: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let p = Path::new(&args.root_dir);

    let operations = bulk_renamer::bulk_rename(p, &args.existing, &args.new_pattern)?;
    for new_name in operations {
        println!("Renamed to {new_name}");
    }

    Ok(())
}
