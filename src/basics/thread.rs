use std::thread;

pub(crate) fn l_thread() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    thread::scope(|scop| {
        scop.spawn(|| {
            println!("Numbers: {}", numbers.len());
        });

        scop.spawn(|| {
            println!("Numbers: {}", numbers.len());
        });
        
        let scop_thread = thread::current().id();
        println!("{:?}", scop_thread);
    });

    let main_thread_id = thread::current().id();
    
    println!("{:?}", numbers.len());
    println!("{:?}", main_thread_id);

    let _handle= thread::Builder::new()
    .name("tid__1".to_string())
    .spawn(move || {
        println!("Other thread. Thread ID: {:?}", thread::current().id());
        println!("Numbers length: {}", numbers.len());
    });

    // Does not work. Because numbers is moved to tid__1
    // thread::spawn(move || {
    //     println!("Other thread. Thread ID: {:?}", thread::current().id());
    //     println!("Numbers length: {}", numbers.len());
    // });
}

