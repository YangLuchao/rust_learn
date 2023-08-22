use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    let result: Vec<&str> = if config.ignore_case {
        println!("case");
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn _new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("params error!")
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config { query, file_path, ignore_case }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case
        })
    }
}

/**
 * 从字符串中查找子串，并返回
 * query 被查找的子串
 * contains 字符串
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    // 将一句话拆成一行一行的
    for line in contents.lines() {
        // 是否包含数据
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    //let query: String = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
    Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
