use std::env;
// 需要包含无效的Unicode字符的参数时， 使用 std::env::args_os 代替 std::env::args

use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // 注意使用collect()时，最好是注明数据类型
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("failed to load config, err: {:?}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("app error: {:?}", err);
        process::exit(1);
    }
}

fn _parse_flag(args: &[String]) -> Config {
    Config {
        // 这里强调下为什么要使用clone(),另一方面，尽管这种方式不太高效，相对于储存字符串数据的引用会消耗更多的时间和内存，也无需管理引用的生命周期，但换取的是更简洁的语意
        // todo 换成引用，并引出迭代器，以及如何更高效的处理这个
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}

// 几个版本的引进，将config的实现从命令行的返回元组到结构体，再到结构体的方法，再到接口的实现
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("query: {:?}, content: {:?}", config.query, content);
    Ok(())
}
