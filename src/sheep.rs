


struct Sheep {
    naked:bool,
    name: String
}

impl Sheep{
    fn is_naked(&self) -> bool{
        self.naked
    
    }
    fn shear(&mut self){
        if self.is_naked(){
            println!("{} is already naked...",self.name());
        }else{
            println!("{} gets a haircut!",self.name);
            self.naked = true;
        }
    }
}

trait Animal{
    fn new(name:String) ->Self;

    fn name(&self) -> String;

    fn noise(&self) -> String;

    fn talk(&self){
        println!("{} says {}",self.name(),self.noise());
    }
}

impl Animal for Sheep{
    fn new(name:String) ->Sheep{
        Sheep{name:name,naked:false}
    }

    fn name(&self) ->String{
        self.name.clone()
    }

    fn noise(&self) -> String{
        if self.is_naked(){
            "baaaaaaaah?".to_string()
        }else{
            "baaaaaaaaah!".to_string()
        }
    }

    fn talk(&self){
        println!("{} pauses briefly....{}",self.name(),self.noise());
    }
}



pub fn test_sheep(){
    let mut dolly:Sheep = Animal::new("Dolly".to_string());
    dolly.talk();
    dolly.shear();
    dolly.talk();
}