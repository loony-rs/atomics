use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn some_work() {
    thread::sleep(Duration::from_secs(1));
}

pub fn l_load_store_bool() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let bg_thread = thread::spawn(||{
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => {
                println!("Help!");
            }
            "stop" => {
                break;
            }
            _ => {}
        }
    }

    STOP.store(true, Relaxed);
    bg_thread.join().unwrap();
}

pub fn l_load_store_usize() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|scope|{
        scope.spawn(||{
            for i in 0..5 {
                some_work();
                num_done.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 5 {
                break;
            }
            println!("Working... {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
        println!("Complete");
    });
}