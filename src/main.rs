use std::net::IpAddr;



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






use crate::garden::vegetables::Asparagus;


pub mod garden;



fn test_crate(){
    println!("test_crate");
    let plant = Asparagus {};
}

fn main() {
//    test_struct();
//    test_enum();
    test_crate();

}
