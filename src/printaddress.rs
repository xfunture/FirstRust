
fn print_integer_address(value:&i32){
    println!("function integer address:{:p}",&value);
    println!("function integer data address:{:p}",&&value);
}


fn print_string_address(value:&String){
    println!("function string address:{:p}",&value);
    println!("function string data address:{:p}",value.as_ptr());
}

pub fn print_address(){
    let s:String = String::from("hello,world");
    println!("string address:{:p}",&s);
    println!("string data address:{:p}",s.as_ptr());

    print_string_address(&s);


    let integer:i32 = 32;
    println!("integer address:{:p}",&integer);
    println!("integer data address:{:p}",&&integer);
    print_integer_address(&integer);

    //通过测试发现，字符串字面值存储在堆上，变量存在栈上，整型无论是变量还是值都存储在栈上


}