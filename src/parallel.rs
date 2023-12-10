use std::thread;
use std::time::Duration;




pub fn test_thread(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        for i in 1..10{
            println!("here is vector:{:?}",v);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5{
        println!("hi number {} from the main thread",i);
        thread::sleep(Duration::from_millis(1));
    }

}

pub fn test_thread_v1(){
    let new_thread = thread::spawn(move || {
           thread::spawn(move || {
                loop{
                    println!("I'm a new thread.")
                }
           })
    });
        new_thread.join().unwrap();
        println!("Child thread is finish!");
        thread::sleep(Duration::from_millis(1000*10));
}   