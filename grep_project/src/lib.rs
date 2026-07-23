pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub struct Config<'a> {
    pub query: &'a str,
    pub path_name: &'a str,
}

impl<'a> Config<'a> {
    pub fn check_args(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else {
            let query = args[1].as_str();
            let path_name = args[2].as_str();
            Ok(Config { query, path_name })
        }
    }
    // this returs a success or a failure thats why we use Result type
}
