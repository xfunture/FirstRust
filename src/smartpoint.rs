use std::ops::Deref;


//测试Deref 解引用
struct MyBox<T>(T);

impl<T> MyBox<T>{
        fn new(x:T) -> MyBox<T>{
            MyBox(x)
        }

}
 impl<T> Deref for MyBox<T>{
            type Target = T;

            fn deref(&self) -> &Self::Target{
                &self.0
            }
 }


 fn display(s:&str){
    println!("{}",s);
}

use std::rc::Rc;
use std::thread;
use std::sync::Arc;
use std::cell::Cell;
use std::cell::RefCell;


//测试引用计数(Reference counting) Rc
//适用于单线程场景
//Rc是不可变引用，无法修改它的值，只能进行读取
//Rc<T> 是一个智能引用，实现了Deref 特征
fn test_rc(){

    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b =  Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c =  Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//Atomic Reference Counting 原子性引用计数
//只能读取，不能修改
//线程安全，可以用与多线程场景

fn test_arc(){
    let s = Arc::new(String::from("hello,world"));
    for i in 0..10{
        let s = Arc::clone(&s);
        let handle = thread::spawn(move ||{
            println!("i:{} {}",i,s);
        });
    }
}



//测试内部可变性
fn test_cell(){
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{:?},{},{}",c,one,two);
}

//定义在外部库中的特征
pub trait Messager{
    fn send(&self,msg:String);
}

struct MsgQueue{
    msg_cache:RefCell<Vec<String>>
}

impl Messager for MsgQueue{
    fn send(&self,msg:String){
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn test_rc_refcell(){
    let s = Rc::new(RefCell::new("hello,world".to_string()));
    let s1 = s.clone();
    let s2  = s.clone();
    s2.borrow_mut().push_str(" rick!");
    println!("{:?}\n{:?}\n{:?}\n",s,s1,s2);

}
pub fn test_smart_point(){

    let s = String::from("hello,world");
    display(&s);

    let y = MyBox::new(5);
    assert_eq!(5,*y);
    println!("{:?}",*y);

    // let s= String::from("hello,world");
    // let a = Box::new(s);
    // let b = Box::new(s);         
    // println!("a:{}",a);
    test_arc();
    test_cell();
    test_rc_refcell();

}


