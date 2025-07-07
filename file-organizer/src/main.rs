use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let target_dir = Path::new(&args.path);

    if !target_dir.is_dir() {
        eprintln!("Provided path is not a directory.");
        return Ok(());
    }

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let subdir = target_dir.join(ext);
                if !subdir.exists() {
                    fs::create_dir(&subdir)?;
                }

                let filename = path.file_name().unwrap();
                let new_path = subdir.join(filename);

                fs::rename(&path, &new_path)?;
                println!("Moved {:?} → {:?}", path, new_path);
            }
        }
    }

    println!("✅ Done organizing files in {}", args.path);
    Ok(())
}
