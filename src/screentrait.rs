

pub trait Draw{
    fn draw(&self);
}


pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}

impl Draw for Button{
    fn draw(&self){
        println!("Button draw");
    }
}
struct SelectBox{
    width:u32,
    height:u32,
    options:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("SelectBox");
    }
}

pub struct Screen{
    pub componets:Vec<Box<dyn Draw>>
}


impl Screen{
    pub fn run(&self){
        for component in self.componets.iter(){
            println!("run");
            component.draw();
        }
    }
    pub fn add(&mut self,draw:Box<dyn Draw>){
        self.componets.push(draw);
    }
}

fn draw1(x:Box<dyn Draw>){
    x.draw();
}

fn draw2(x:&dyn Draw){
    x.draw();
}




pub fn test_screen_trait(){
    let button:Button =  Button{width:20u32,height:30u32,label:String::from("button")};
    button.draw();
    let selectbox = SelectBox{width:10u32,height:30u32,options:vec!["hello,world,selectbox".to_string()]};
    selectbox.draw();

    draw2(&button);
    draw2(&selectbox);

    let mut screen = Screen{componets:vec![]};
    screen.add(Box::new(Button{width:20u32,height:30u32,label:String::from("button")}));
    screen.add(Box::new(button));
    screen.add(Box::new(selectbox));
    screen.run();


}



