use std::{collections::HashSet, io::prelude::*};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: path_zipper <output_path> <input_path:zip_path...>");
        std::process::exit(1);
    }
    match run(args) {
        Ok(_) => {
            println!("Ok");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
fn run(args: Vec<String>) -> std::io::Result<()> {
    let path = std::path::Path::new(&args[1]);
    let file = std::fs::File::create(&path).unwrap();
    let added_files = added_files(&args);

    let directories: HashSet<&str> = added_files
        .iter()
        .flat_map(|(_, internal_path)| internal_path.rfind('/').map(|idx| &internal_path[0..idx]))
        .collect();

    let mut zip = zip::ZipWriter::new(file);

    for dir in directories {
        zip.add_directory(dir, Default::default())?;
    }
    for (system_path, zip_path) in added_files {
        zip.start_file(zip_path, Default::default())?;
        let mut file = std::fs::File::open(system_path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        zip.write_all(&buf)?;
    }

    zip.finish()?;
    Ok(())
}

fn added_files<'a>(args: &'a Vec<String>) -> Vec<(&'a str, &'a str)> {
    let mut result = Vec::new();
    for arg in &args[2..] {
        if let Some(pos) = arg.find(':') {
            result.push((&arg[0..pos], &arg[pos+1..]));
        } else {
            result.push((&arg, &arg));
        }
    }

    result
}
