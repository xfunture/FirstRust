trait Draw{
    fn draw(&self) -> String;
}

impl Draw for u8{
    fn draw(&self) -> String{
        format!("u8:{}",*self)
    }
}


impl Draw for f64 {
    fn draw(&self) -> String{
        format!("f64:{}",*self)
    }
}


fn draw_with_box(x:Box<dyn Draw>) -> String{
    x.draw()
}

fn draw_with_ref(x:&dyn Draw) -> String{
    x.draw()
}


pub fn test_dyn_box(){
    let x = 1.1f64;
    let y = 8u8;
    //draw x
    let output:String = draw_with_box(Box::new(x));
    println!("output:{:?}",output);

    //draw y
    let output:String = draw_with_ref(&y);
    println!("output:{:?}",output);
    println!("Success!");

}
