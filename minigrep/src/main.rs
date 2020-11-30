use std::env;
// 需要包含无效的Unicode字符的参数时， 使用 std::env::args_os 代替 std::env::args
use minigrep::Config;
use std::process;

fn main() {
    // 注意使用collect()时，最好是注明数据类型
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("failed to load config, err: {:?}", err);
        process::exit(1);
    });

    if let Err(err) = Config::run(config) {
        eprintln!("app error: {:?}", err);
        process::exit(1);
    }
}
