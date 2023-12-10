
use std::thread;
struct Cache<T>
where  T:Fn(u32) -> u32
{
    query:T,
    value:Option<u32>,
}



impl<T> Cache<T>
where T:Fn(u32) -> u32,
{
    fn new(query:T) -> Cache<T>{
        Cache{
            query,
            value:None
        }
    }
    
    fn value(&mut self,arg:u32) ->u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn fn_once<F>(func:F)
where F:FnOnce(usize) -> bool + Copy,
{
    println!("{}",func(3));
    println!("{}",func(4));
}


//闭包作为函数返回值
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32>{
    let num = 5;
    if x>1{
        Box::new(move |x| x + num)
    }else{
        Box::new(move |x | x-num)
    }
}

pub fn test_closure(){
    let sum = |x,y| x+y;
    let x = 1;
    let y = 2;
    let  output = sum(x,y);
    println!("sum:{}",output);

    let add_fun = |x:u32,y:u32| -> u32 {x+y};
    let output = add_fun(x,y);
    println!("sum:{}",output);


    //三种Fn特征:FnOnce ,Fn,FnMute
    //一个闭包并不仅仅实现了某一种Fn特征，规则如下
    //(1) 所有闭包都自动实现了FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    //(2) 没有移出所捕变量的所有权的闭包自动实现了FnMut特征
    //(3) 不需要对所捕获变量进行改变的闭包自动实现了Fn特征
    //1.FnOnce 该类型的闭包会拿走被捕获变量的所有权
    let x = vec![1,2,3];
    fn_once(|z| {z == x.len()});


    //move 会强制闭包会捕获环境中的值的所有权
    //如果不添加move 则会报错，因为无法确定新线程的生命周期和原来线程array 的生命周期谁长
    let array = vec![1,2,3];
    let handle = thread::spawn(move || {println!("Here's a vector:{:?}",array)});
    handle.join().unwrap();

    //2.Fnmut 以可借用的方式捕获了环境中的值，因此可以修改该值
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}",s); 

    let f  = factory(2);
    let answer = f(1);
    assert_eq!(6,answer);

}