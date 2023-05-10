use clap::Parser;
use std::env;
use std::path::Path;

/// Bulk rename all files recursively with a string substitution.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root directory
    #[arg(short, long, required = false)]
    root_dir: Option<String>,

    /// Original pattern
    #[arg(short, long)]
    original: String,

    /// Updated pattern
    #[arg(short, long)]
    update: String,

    /// Dry-run. Enable to avoid mutating file system.
    #[arg(short, long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path_name = &args.root_dir.map_or(
        env::current_dir().expect("Unable to determine current working directory"),
        |p| p.into(),
    );

    let p = Path::new(path_name);

    let operation = if args.dry_run {
        bulk_renamer::Operation::DryRun
    } else {
        bulk_renamer::Operation::Mutating
    };

    let operations = bulk_renamer::bulk_rename(p, &args.original, &args.update, &operation)?;

    for names in operations {
        match operation {
            bulk_renamer::Operation::DryRun => {
                println!(
                    "{} would have been renamed to {}.",
                    names.get_original_name(),
                    names.get_updated_name()
                );
            }
            bulk_renamer::Operation::Mutating => {
                println!(
                    "Renamed {} to {}",
                    names.get_original_name(),
                    names.get_updated_name()
                );
            }
        }
    }

    Ok(())
}
