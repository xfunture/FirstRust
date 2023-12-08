
use std::ops::Add;

//为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T:Add<T,Output = T>>{  //限制类型T必须实现了Add特征，否则无法进行操作。
    x:T,
    y:T,
}


impl <T:Add<T,Output = T>> Add for Point<T>{
    type Output = Point<T>;
    fn add(self,p:Point<T>) -> Point<T>{
        Point { x: self.x + p.x, y: self.y + p.y }
    }
}


fn add<T:Add<T,Output=T>>(a:T,b:T) -> T
{
    a + b
}

pub fn test_add(){
    let p1 = Point{x:1.1f32,y:1.1f32};
    let p2 = Point{x:2.1f32,y:2.1f32};
    println!("{:?}",add(p1,p2));

    let p3 = Point{x:1i32,y:1i32};
    let p4 = Point{x:2i32,y:2i32};
    println!("{:?}",add(p3,p4));
}