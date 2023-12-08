use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
use std::fmt::{Display, Debug};
use std::fmt;
mod add;            //声明add模块
use add::test_add;  //引入add 模块函数


mod displlay;
use displlay::test_display;


fn another_function(x:i32,y:char) {
    println!("Another function:{} {}", x,y);
}

fn test_let_mut(){
     let mut x = 6;
     print!("Testing x:{}\n",x);
    x = 10;
    print!("Testing x:{}\n",x);
    let (x,mut y):(bool,bool) = (true,true);
    println!("a = {:?}, b = {:?}",x,y);
    y = true;
    assert_eq!(x,y);


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let array : [i32;5] = [1,2,3,4,5];
    println!("array: {:?}",array);

    let char_value : char = 'x';
    another_function(x,char_value);
}


fn greet_world(){
    let southern_germy = "Grüß Gott!\n";
    let chinese = "世界，你好\n";
    let english = "Hello,world\n";
    let regions = [southern_germy,chinese,english];
    for region in regions.iter(){
        print!("{}",&region);
    }
}


fn test_loop(){
    let mut counter = 0;
    let result = loop {counter+=1;
    if counter == 10{
        break counter * 2;
    }   
    };
    println!("counter:{}",counter);
    println!("result:{}",result);
}

fn test_loop_v1() {
    println!("----------------------------------------------------------------");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0{
        println!("{}", number);
        number -= 1;
    }

    let a = [10,9,8,7,6,5,4,3,2,1,0];
    let mut index = 0;
    while index < 5{
        println!("The value is {}",a[index]);
        index += 1;
    }

    for element in a{
        println!("The value is {}",element);
    }

    for number in (1..10).rev()
    {
        println!("1 The value is {}",number);
    }

    let s = "hello,world";
    println!("The string is {}",s);

    let mut s1 = String::from("hello,world");
    println!("The String is {}",s1);
    s1.push_str("rick");
    println!("The String is {}",s1);

    let s2 = s1.clone();
    println!("The String is {}",s2);
}

fn take_owership(some_string:String)->String{
    some_string
}

fn takes_and_gives_back(a_string:String)->String{
    a_string
}

fn gives_ownership()->String{
    let some_string = String::from("yours");
    some_string
}

fn makes_copy(some_integer: i32){
    println!("{}",some_integer);
}

fn calculate_length(s:String)->(String,usize){
    let length = s.len();
    (s,length)
}

fn calculate_length_v1(s:&String)->usize{
    s.len()
}


fn calculate_length_v2(s:&String)->(&String,usize){
    (&s,s.len())
}

#[derive(Debug)]
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}




#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self,other:&Rectangle) -> bool{
    self.width > other.width && self.height > other.height
    }

    //关联函数，用于构造对象
    fn square(width : u32,height : u32) ->Self{
        Self {
            width:width,
            height:height
        }
    }
}


fn build_user(email:String,username:String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn test_struct(){

    let mut user1 = User{
            active : true,
            username : String::from("rick"),
            email : String::from("rickbruce1992@gmail.com"),
            sign_in_count : 1,

    };
    user1.email = String::from("15768184216@163.com");
    let mut user2:User = build_user(String::from("test@163.com"), String::from("xiejinhao"));

    println!("User:{:#?}",user2);

    let scale = 2;
    let rect1 = Rectangle{
        width:dbg!(60 * scale),
        height:100,
    };

    dbg!(&rect1);
    println!("rect is {:#?}",rect1);
    println!("The area of the rectangle is {} square pixels",rect1.area());

    let rect2 = Rectangle{
        width:30,
        height:50
    };

    let rect3:Rectangle = Rectangle { width: 10, height: 40 };
    let rect4:Rectangle = Rectangle {width:60,height:45};

    println!("rect1 can hold rect2:{}",rect1.can_hold(&rect2));
    println!("rect1 can hold rect3:{}",rect1.can_hold(&rect3));
    println!("rect1 can hold rect4:{}",rect1.can_hold(&rect4));


    let rect5 = Rectangle::square(3,4);
    println!("rect5 {:#?}",rect5);

}



#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}
#[derive(Debug)]
struct IPAddr{
    kind: IpAddrKind,
    address: String,
}

enum IPAddrEnum{
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangerColor(i32,i32,i32)
}

impl Message{
    fn call(&self){
        // println!("call message write:{}",self.Write);
    }
}
#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_in_cents(coin:Coin) ->u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_enum(){
    let four  = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IPAddr{kind:IpAddrKind::V4,address:String::from("127.0.0.1")};
    let lookback: IPAddr = IPAddr { kind: IpAddrKind::V6, address:String::from("::1") };
    println!("IpAddr : {:#?}",home);
    println!("IpAddr : {:#?}",lookback);

    let new_home = IPAddrEnum::V4(127,0,0,1);
    let new_looback:IPAddrEnum = IPAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("hello,rick"));
    m.call();

    let some_number = Some(5);

    let some_char = Some('e');

    let absend_number:Option<i32> = None;

    value_in_cents(Coin::Quarter(UsStates::Alaska));

    let five = Some(5);
    let six: Option<i32> = plus_one(five);
    let none = plus_one(None);

    let coin = Coin::Quarter(UsStates::Alabama);
    let mut count =  0;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {:?}",state);
    }else{
        count += 1;
    }

}






use FirstRust::eat_at_restaurant;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn test_crate(){
    println!("test_crate");
    let plant = Asparagus {};
    let first_string: String = String::from("hello,world");
    plant.functions(first_string);
}

fn test_vector(){
    println!("test_vector:\n");
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    let first = &v[0];
    println!("test_vector:{:?}",v);
    let third = &v[2];
    v.push(10);
    let mut third:Option<&i32> = v.get(100);
    match third{
        Some(third) => {println!("The third elements is {}",third)},
        other => println!("There is no third element"),
    }

    let mut x = vec![1, 2, 3, 4, 5];

    let second = &x[0];

    x.push(6);

    let mut y = vec![1,2,3,4,5,6];
    for i in &mut v{
        *i *= 50;
        println!("i:{}",i);
    }

    for i in v{
        println!("i:{}",i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello,world")),
        SpreadsheetCell::Float(10.12)
    ];

    row.remove(2);
    println!("v:{:?}",row);

    let s1 = String::from("hello,");
    let s2: String = String::from("world");
    let s3 = s1 + &s2;
    println!("s3:{}\n",s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = format!("{s4}-{s5}-{s6}");
    println!("{s7}");

}



fn test_hashmap(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score:{}",score);

    scores.insert(String::from("Blue"),100);
    scores.entry(String::from("Yellow")).or_insert(9000);

    for (key,value) in &scores{
        println!("{key} {value}");
    }
    let mut array = vec![1,2,3,4,5];

}



fn test_read_file(){
    let greeting_file_result = File::open("temp.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the File:{}",error),
    };
}


fn test_read_file_v1(){
    let greeting_file_result = File::open("temp.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("temp.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}",e),
            },
            other_error => {
                panic!("Problem opening the file:{:?}",other_error);
            }
        },
    };
    println!("greeting_file:{:?}",greeting_file);
}

fn test_read_file_v2(){
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("greeting_file:{:?}",greeting_file);

}

fn read_username_from_file() -> Result<String,io::Error>{
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v1() -> Result<String,io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
}

fn test_username_from_file(){
    let result = read_username_from_file_v1();
    if result.is_ok(){
        println!("result:{:?}",result);
    }else{
        panic!("Result:{:?}",result);
    }
}


fn largest(list:&[i32]) -> &i32{

    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}


struct Point<T,U>{
    x:T,
    y:U,
}

impl<T,U> Point<T,U>{
    fn getx(&self) -> &T{
        &self.x
    }
}

impl Point<f32,f32>{
    fn distance_from_origin(&self) ->f32{
        (self.x.powi(2) * self.y.powi(2)).sqrt()
    }
}

fn largest_char(list:&[char]) -> &char{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}


// fn largest_generics<T>(list:&[T]) -> &T{
//     let mut largest = &list[0];
//     for item in list{
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

fn find_largest(){
    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    println!("The largest number is {}",result);
    let number_list = vec![102,34,6000,89,54,2,43,8];
    let result = largest(&number_list);
    println!("The largest number is {}",result);

    let char_list = vec!['y','m','a','q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}",result);

}

fn test_struct_geneic(){
    let both_integer = Point{x:5,y:10};
    let both_float = Point{x:3.0,y:5.0};
    let integer_and_flooat = Point{x:4,y:32.0};
}


pub trait Summary{
    fn summarize(&self) -> String{
        String::from("Read more ...... ")
    }
}

pub struct Post{
    pub title:String,   //标题
    pub author:String,  //作者
    pub content:String, //内容
}

impl Summary for Post{}


#[derive(Debug)]
pub struct Weibo{
    pub username:String,
    pub content:String,
    pub age:i32,
}


//重载了特征Summary的方法summarize
impl Summary for Weibo{
    fn summarize(&self) -> String {
        format!("{}发表了微博{}",self.username,self.content)
    }
}


//为Weibo 实现了Display 特征
impl fmt::Display for Weibo{
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> Result<(),std::fmt::Error>{
            write!(f,"display fmt: I'm {},I {} years old,{}",self.username,self.age,self.content)
    }
}

//参数类型：实现了summary 特征的类型（trait summary）,特征函数类型
//任何实现了summary 特征的的类型都可以作为该函数的参数
fn summary_function(item: &impl Summary){
    println!("Breaking news!{}",item.summarize());
}


pub fn notify<T:Summary>(item1:&T , item2:&T){
    println!("notify Breaking news1:{}",item1.summarize());
    println!("notify Breaking news2:{}",item2.summarize());
}

pub fn notify_v1(item: &(impl Summary + Display))
{
    println!("notify_v1 Breaking news:{}",item);
}

pub fn notify_v2<T:Summary + Display>(item:&T)
{
    println!("notify_v2 Breaking news:{}",item);
}

// fn some_function<T:Display + Clone,U:Clone + Debug>(t:&T,u:&U) -> i32{
// }

//通过where简化trait bound
// fn some_function<T:,U>(t:&T,u:&U) -> i32
// where T:Display + Clone,
//       U:Clone + Debug
// {

// }


//返回实现了trait 的类型
fn returns_summarizable(switch:bool) -> impl Summary{
    if switch{
            Post{
                title:String::from("Penguins win the Stanley Cup Championship!"),
                author:String::from("Iceburgh"),
                content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
            }
        }else{
            Post{
                title:String::from("Penguins win the Stanley Cup Championship!"),
                author:String::from("Iceburgh"),
                content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
            }

        //     Weibo{
        //         username:String::from("Xiejinhao"),
        //         content:String::from("hello"),age:31
        //     }
        }
}



fn test_trait(){
    let post = Post{title:String::from("Post title"),author:String::from("rick"),content:String::from("hello,world")};
    let summarize = post.summarize();
    println!("Post summarize:{}",summarize);

    let weibo = Weibo{username:"rickbruce".to_string(),content:"hello,world".to_string(),age:18};
    let summarize:String = weibo.summarize();
    println!("Post summarize:{}",summarize);

    summary_function(&post);
    summary_function(&weibo);
    notify(&post,&post);      //item1 和item2 必须是相同的类型，同时T:Summary 说明了必须实现Summary 特征
    notify_v1(&weibo);
    notify_v2(&weibo);
    let post = returns_summarizable(true);
    let summarize:String = post.summarize();
    println!("Post summarize:{}",summarize);


}



mod sheep;
use sheep::test_sheep;

mod traitobject;
use traitobject::test_traitobject;

mod traitarray;
use traitarray::test_traitvec;

mod dynbox;
use dynbox::test_dyn_box;

mod traitstudent;
use traitstudent::test_trait_student;

mod screentrait;
use screentrait::test_screen_trait;

mod printaddress;
use printaddress::print_address;

mod static_dynamic_dispatch;
use static_dynamic_dispatch::test_static_and_dynamic_dispatch;

mod safeobject;
use safeobject::test_safe_object;

mod supertrait;
use supertrait::test_supertrait;
// use supertrait::test_reference;

fn longest<'a>(x:&'a i32,y:&'a i32) -> &'a i32{
    if x>=y{
        x
    }else{
        y
    }
}



fn test_lifetime(){
    let x = 3;
    let y = 4;
    let z= longest(&x, &y);
    println!("largest value:{}",z);
}

fn main() {
//    test_struct();
//    test_enum();
    // test_crate();
    // test_vector();
    // test_hashmap();
    // test_read_file();
    // test_read_file_v1();
    // test_read_file_v2();
    // test_username_from_file();
    // find_largest();
    // test_struct_geneic();
    test_trait();                           // 测试特征类型，特征约束
    test_add();                             // 测试自定义类型实现 add 操作
    test_display();                         // 自定义类型打印输出
    test_sheep();                           // 测试特征类型
    test_traitobject();                     // 测试函数返回同特征的不同对象
    test_traitvec();                        // 测试数组存储不同的特征对象
    test_dyn_box();
    test_trait_student();               
    test_screen_trait();                    // 测试Box 和 dyn
    print_address();                        // 打印变量地址,字符串字面值存储在堆上
    test_static_and_dynamic_dispatch();     // 测试静态分发和动态分发
    test_safe_object();
    test_supertrait();                      //测试超级trait
    // test_reference();
    test_lifetime();
}
