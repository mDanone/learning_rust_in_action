use std::thread;

fn main() {
    let mut counter = 0;
    let mut cloned_counter = counter;
    
    {
        let first_thread = thread::spawn(move || {
            counter += 1;
            counter
        });
        let second_thread = thread::spawn(move || { 
            cloned_counter += 1;
            cloned_counter
        });
        counter = first_thread.join().expect("Counter not work");
        cloned_counter = second_thread.join().expect("Counter not work");
    
        println!("{}", counter + cloned_counter);
    }

    let mut some_list = Vec::from("Hello");
    let mut r2 = &mut some_list;
    
    let mut r1 = &mut some_list;
    println!("{:?}", r1);


    let mut r2 = &mut some_list;
    println!("{:?}", r2);

    
    println!("{:?}", r1);
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

