use std::collections::VecDeque;

trait Bird{
    fn quack(&self);
}

struct Duck;

impl Duck{
    fn fly(&self){
        println!("Look,the duck is flying")
    }
}

struct Swan;
impl Swan{
    fn fly(&self){
        println!("Look,the duck.. oh sorry,the swan is flying")
    }
}

impl Bird for Duck{
    fn quack(&self){
        println!("{}","duck duck");
    }
}


impl Bird for Swan{
    fn quack(&self){
        println!("{}","swan swan");
    }
}

//测试数组存储特征对象
pub fn test_traitvec(){
    // let birds = [Duck{},Swan{}];
    let birds : [Box<dyn Bird>;2] = [Box::new(Swan{}),Box::new(Duck{})];
    for bird in birds{
        bird.quack();
    }
}