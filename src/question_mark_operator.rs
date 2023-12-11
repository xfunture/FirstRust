
use std::num::ParseIntError;



fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24".parse()?; // 立即返回一个 Err()
    Ok(x + y)                    // 不会执行到这里
}



pub fn test_question_mark_operator(){
    let res = try_to_parse();
    println!("{:?}",res);
}