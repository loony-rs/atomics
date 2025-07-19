use std::{thread, sync::atomic::AtomicU32};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};

fn some_work() {
    thread::sleep(Duration::from_secs(1));
}

pub fn l_fetch_add() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|scope| {
        for t in 0..4 {
            scope.spawn(move || {
                for i in 0..25 {
                    let ins = Instant::now();
                    some_work();
                    let elapsed = ins.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(elapsed, Ordering::Relaxed);
                    max_time.fetch_max(elapsed, Ordering::Relaxed);
                }
            });
        }

        loop {
            let t_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let m_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 { break; }
            if n == 0 { 
                println!("Started...") 
            } else {
                println!(
                    "Working... {n}/100 done, {:?} average, {:?} peak",
                    t_time/n as u32,
                    m_time
                )
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}