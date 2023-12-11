

use std::fs::File;
use core::num::ParseIntError;

fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123dff".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // 立即返回一个 Err()
    Ok(x + y)                    // 不会执行到这里
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

pub fn test_unwrap(){
    // let file = File::open("/home/rick/PycharmProjects/FirstRust/hello.txt").unwrap();        //unwrap 可以简化错误处理，成功返回文件句柄，失败则调用panic!宏

   let file = File::open("/home/rick/PycharmProjects/FirstRust/hello.txt").expect("hello.txt not found");   //expect 和unwrap 会产生相同的效果，但是expect 可以接收错误信息

    let mut username = String::new();
//    let mut file = File::open("/home/rick/PycharmProjects/FirstRust/hello.txt")?.read_to_string();   //expect 和unwrap 会产生相同的效果，但是expect 可以接收错误信息

    println!("file:{:?}",file);

    let value = try_to_parse();
    println!("{:?}",value);

}