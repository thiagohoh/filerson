use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn file_walk(
    dir: &Path,
    file: &fs::File,
    exclude_list: &Vec<String>,
    no_extension: bool,
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let extension = match path.extension() {
                None => String::from("-"),
                Some(s) => match s.to_str() {
                    None => String::from("-"),
                    Some(s) => s.to_string(),
                },
            };

            if path.is_file() {
                if starts_with_dot(&entry)
                    || exclude_list.contains(&extension)
                    || skip_empty(&extension, no_extension)
                {
                    continue;
                }
                file_writer(entry, file);
            } else {
                dir_writer(entry, file);
                if let Err(e) = file_walk(&path, file, exclude_list, no_extension) {
                    println!("err {e}");
                }
            }
        }
    }
    Ok(())
}

fn skip_empty(extension: &String, no_extension: bool) -> bool {
    if extension.contains('-') && no_extension {
        true
    } else {
        false
    }
}

fn starts_with_dot(entry: &fs::DirEntry) -> bool {
    if entry.file_name().to_str().unwrap().starts_with('.') {
        true
    } else {
        false
    }
}

fn dir_writer(entry: fs::DirEntry, mut file: &fs::File) {
    if let Err(e) = writeln!(
        file,
        "--------dir: {entry} -------",
        entry = entry.file_name().to_str().unwrap()
    ) {
        println!("Error {e}");
    }
}

fn file_writer(entry: fs::DirEntry, mut file: &fs::File) {
    if let Err(e) = writeln!(file, "{file}", file = entry.file_name().to_str().unwrap()) {
        println!("Could'n write to file {e}");
    }
}
