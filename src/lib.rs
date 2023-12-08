mod front_of_house;
use crate::front_of_house::hosting;
pub fn eat_at_restaurant(){
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    //相对路径
    // front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_food(){
    
}

fn server_order(){}



pub mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::server_order();
    }

    fn cook_order(){

    }

}
