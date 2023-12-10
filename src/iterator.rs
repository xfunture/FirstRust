use std::collections::HashMap;

#[derive(Debug)]
struct Shoe{
    size:u32,
    style:String,
}

fn shoes_in_size(shoes:Vec<Shoe>,shoe_size:u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

//into_iter 会夺取变量的所有权
//iter_mut 是可变借用
//iter 是借用

pub fn test_iterator(){
    let values = vec![1,2,3,4];
    for v in values.into_iter(){
        println!("{}",v);
    }
    // println!("{:?}",values);                                   //这句代码会报错，因为into_iter 会夺取values的所有权
    let values = vec![1,2,3,4];
    for v in values.iter(){
        println!("{}",v);
    }
    println!("{:?}",values);

    let mut values = vec![1,2,3];
    let mut value_iter_mut = values.iter_mut();
    if let Some(v) = value_iter_mut.next(){
        println!("Some:{}",v);
        *v = 0;
    }
    println!("{:?}",values);


    //消费者适配器,它会消费掉迭代器中的元素，然后返回其类型的值，
    //只要迭代器上的某个方法 A 在其内部调用了 next 方法，那么 A 就被称为消费性适配器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    assert_eq!(total,6);
    println!("v1:{:?}",v1);
    // println!("v1_iter:{:?}",v1_iter); 代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权


    // 迭代器适配器
    // 迭代器适配器，顾名思义，会返回一个新的迭代器，这是实现链式方法调用的关键：v.iter().map().filter()...。
    let v1 = vec![1,2,3];
    let v2:Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("collect:{:?}",v2);

    //sum 消费者适配器
    //collect 消费者适配器
    //zip 迭代器适配器，
    //filter 迭代器适配器，用于对迭代器中的每个值进行过滤
    //map 迭代器适配器
    //enumerate() 
    let names = ["sunface","sunfei"];
    let ages = [18,18];
    let folks:HashMap<_,_> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}",folks);

    let s1:String = String::from("hello");
    let s2:String = String::from("wolrd4");
    let v8 :Vec<Shoe> = vec![Shoe{size:s1.len() as u32,style:s1},Shoe{size:s2.len() as u32,style:s2}];
    let v9 :Vec<Shoe> = shoes_in_size(v8,5u32);
    println!("v9:{:?}",v9);
 
    let v = vec![1u64,2,3,4,5,6];
    let val = v.iter()
                                                    .enumerate()
                                                    .filter(|&(idx,_)| idx % 2 == 0)
                                                    .map(|(_,val)| val )
                                                    .fold(0u64,|sum,acm| sum + acm);
    println!("val:{}",val );
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().fold(1, |acc, x| acc + x);
    println!("sum:{}",sum);
                                                                                        


}