use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let filename = "hello.txt";
    let f = File::open(filename);

    let _f = match f {
        Ok(file) => println!("{:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(created) => println!("file has benn created: {:?}", created),
                Err(err) => panic!("failed to create file: {:?}", err),
            },
            other_err => panic!("unknown err: {:?}", other_err),
        },
    };

    // 这里可以理解为接收一个函数回调
    let _f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("failed to create file: {:?}", error);
            })
        } else {
            panic!("unknown err: {:?}", error);
        }
    });

    // 简写的两种
    let _f = File::open(filename).unwrap();
    let _f = File::open(filename).expect("failed to open file");

    // let filename = "notExsitFile.txt";
    let name = read_name_from_file(filename);
    println!("name: {:?}", name);

    let name = read_name_from_file_op(filename);
    println!("name: {:?}", name);

    let name = read_name_from_file_chain(filename);
    println!("name: {:?}", name);

    let name = read_name_from_file_crate(filename);
    println!("name: {:?}", name);
}

// read_name_from_file 不使用传播运算符
fn read_name_from_file(filename: &str) -> Result<String, io::Error> {
    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用传播错误的运算符
fn read_name_from_file_op(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 使用常用的链式调用
fn read_name_from_file_chain(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

// 使用内置 fs crate
fn read_name_from_file_crate(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
