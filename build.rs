use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() -> std::io::Result<()> {
    let src_dir = Path::new("src");
    let out_dir = src_dir;
    let mut days = Vec::new();

    // Scan for dayXX.rs files
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                if name.starts_with("day")
                    && path.extension().and_then(|s| s.to_str()) == Some("rs")
                {
                    days.push(name.to_string());
                }
            }
        }
    }

    // Sort days to maintain order
    days.sort();

    // Write the generated module declarations directly into src directory
    let dest_path = Path::new(&out_dir).join("days_mod.inc");
    let mut file = File::create(dest_path)?;

    writeln!(file, "// generated by build.rs")?;
    writeln!(file, "// do not include in source control!")?;
    for day in days {
        writeln!(file, "pub mod {};", day)?;
    }

    Ok(())
}
