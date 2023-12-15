use tokio::time::{sleep,Duration};
use futures;


pub async fn test_select(t1:u64,t2:u64,timeout:u64){
    let f1 = async {
        sleep(Duration::from_millis(t2)).await;
        1
    };

    let f2 = async {
        sleep(Duration::from_millis(t2)).await;
        "hello".to_string();
    };

    let timeout = sleep(Duration::from_millis(timeout));

    tokio::select!{
        _ = timeout => {
            println!("got timeout!");
        }
        v = f1 => {
            println!("got r1: {:?}", v);
        }
        v = f2 => {
            println!("got r2: {:?}", v);
        }
    }

}



async fn borrow_x(x: &u8) -> u8 { *x }

//async 的生命周期
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

pub async fn blocks(){
    let my_string = "foot".to_string();
    
    let future_one = async{
      println!("{my_string}");  
    };

    let future_two = async{
        println!("{my_string}");
    };

    //运行两个Future 直到完成
    let ((),()) = futures::join!(future_one,future_two);
}
