trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}


fn hatch_a_bird(item:i32) -> Box<dyn Bird>{
    if item == 2{
        println!("duck");
        Box::new(Duck{})
    }else{
        println!("swan");
        Box::new(Swan{})
    }

}

// 测试函数返回同特征的不同对象
pub fn test_traitobject()
{
    let duck = Duck{};
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(),"duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(),"swan swan");

    println!("success");
}