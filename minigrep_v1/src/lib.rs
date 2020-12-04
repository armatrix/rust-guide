// 注意这里的环境变量的命名和交互和平时使用是不怎么匹配的，更多的时候我们需要的是一种“完全匹配” 然后才是“模糊匹配”
use std::env;
use std::error::Error;
use std::fs;
// 几个版本的引进，将config的实现从命令行的返回元组到结构体，再到结构体的方法，再到接口的实现
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("no query string"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("no file"),
        };

        // 如果传递命令行参数直接用命令行参数，否则用环境变量，这里不会出错，这样写是为了比如说，只有在传递==1的时候的选择语句
        let is_sensitive = match args.next() {
            Some(_) => true,
            None => env::var("CASE_SENSITIVE").is_err(),
        };

        Ok(Config {
            query,
            filename,
            case_sensitive: is_sensitive,
        })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for result in results {
            println!("{}", result)
        }

        Ok(())
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
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line)
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "aaron";
        let contents = "aaron
        root anderson duct no duct";
        assert_eq!(vec!["aaron"], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "aaron";
        let contents = "aaron
        root anderson duct no duct";
        assert_eq!(vec!["aaron"], search(query, contents))
    }
}
