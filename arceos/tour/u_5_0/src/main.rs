#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::thread;
use std::collections::VecDeque;
use std::sync::Arc;
use std::os::arceos::modules::axsync::spin::SpinNoIrq;

const LOOP_NUM: usize = 64;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Multi-task is starting ...");

    let q1 = Arc::new(SpinNoIrq::new(VecDeque::new()));
    let q2 = q1.clone();

    let worker1 = thread::spawn(move || {
        println!("worker1 ...");
        for i in 0..=LOOP_NUM {
            println!("worker1 [{i}]");
            q1.lock().push_back(i);
            // NOTE: If worker1 doesn't yield, others have
            // no chance to run until it exits!
            thread::yield_now();
        }
        println!("worker1 ok!");
    });

    let worker2 = thread::spawn(move || {
        println!("worker2 ...");
        loop {
            if let Some(num) = q2.lock().pop_front() {
                println!("worker2 [{num}]");
                if num == LOOP_NUM {
                    break;
                }
            } else {
                println!("worker2: nothing to do!");
                // TODO: it should sleep and wait for notify!
                // thread::sleep(Duration::from_secs(3));
                thread::yield_now();
            }
        }
        println!("worker2 ok!");
    });

    println!("Wait for workers to exit ...");
    let _ = worker1.join();
    let _ = worker2.join();

    println!("Multi-task OK!");
}
