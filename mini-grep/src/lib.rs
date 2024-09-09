use std::{env, error::Error, result};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("line: ---------{}", line)
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next(); // 移除第一个参数，因为它是指向二进制文件的路径
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    let query = query.to_lowercase();
    let results: Vec<&'a str> = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();
    results
}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "`
Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn two_result() {
        let query = "duct";
        let content = "`
Rust:
safe, fast, productive.
Duct Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let content = "`
Rust:
safe, fast, productive.
Trust me
            ";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, content)
        )
    }
}
