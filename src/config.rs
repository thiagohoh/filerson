pub struct Config {
    pub path: String,
    pub exclude_file: Vec<String>,
    pub no_extension: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut path = String::from("./");
        let mut exclude_file: Vec<String> = Vec::new();
        let mut no_extension: bool = false;

        if args.len() > 1 {
            if args.contains(&String::from("ne")) {
                no_extension = true;
            }
            for arg in args[1..].iter() {
                if arg.starts_with('.') {
                    let mut extension = arg.to_string();
                    extension.retain(|c| c != '.');
                    exclude_file.push(extension);
                } else {
                    if !arg.to_string().eq(&String::from("ne")) {
                        path = arg.to_string();
                    }
                }
            }
        }
        Ok(Config {
            path,
            exclude_file,
            no_extension,
        })
    }
}
