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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 边界检查
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // 如果传递命令行参数直接用命令行参数，否则用环境变量，这里不会出错，这样写是为了比如说，只有在传递==1的时候的选择语句
        let is_sensitive: bool;
        if args.len() == 3 {
            is_sensitive = env::var("CASE_SENSITIVE").is_err();
        } else {
            is_sensitive = match args[3].parse() {
                Ok(flag) => flag,
                Err(_) => env::var("CASE_SENSITIVE").is_err(),
            };
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
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

fn _parse_flag(args: &[String]) -> Config {
    Config {
        // 这里强调下为什么要使用clone(),另一方面，尽管这种方式不太高效，相对于储存字符串数据的引用会消耗更多的时间和内存，也无需管理引用的生命周期，但换取的是更简洁的语意
        // todo 换成引用，并引出迭代器，以及如何更高效的处理这个
        query: args[1].clone(),
        filename: args[2].clone(),
        case_sensitive: env::var("CASE_SENSTIVE").is_err(),
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // todo 可以用这个例子来说明生命周期
    // let query = query.to_lowercase();
    // let contents = contents.to_lowercase();
    // search(&query, &contents)

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
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
