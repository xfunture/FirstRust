trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// 通过泛型实现静态分发
fn static_dispatch<T:Foo>(value:&T){
    let output = value.method();
    println!("static_dispatch:{}",output);
}

// 通过特征对象实现动态分发
fn dynamic_dispatch1(s:Box<dyn Foo>){
    let output:String = s.method();
    println!("dynamic_dispatch:{}",output);
}

fn dynamic_dispatch2(s:&dyn Foo){
    let output:String = s.method();
    println!("dynamic_dispatch:{}",output);
}

//测试静态分发和动态分发
pub fn test_static_and_dynamic_dispatch() {
    let x = 5u8;
    let y = "Hello,y".to_string();
    let z: String = "hello,x".to_string();

    static_dispatch(&x);
    dynamic_dispatch1(Box::new(y));
    dynamic_dispatch2(&z);

    println!("Success!")
}