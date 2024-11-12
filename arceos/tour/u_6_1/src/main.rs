#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
extern crate axstd as std;
#[macro_use]
extern crate axlog;

use std::collections::VecDeque;
use std::os::arceos::modules::axtask::WaitQueue;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

const LOOP_NUM: usize = 256;
static WQ: WaitQueue = WaitQueue::new();

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    ax_println!("WaitQ is starting ...");

    let q1 = Arc::new(Mutex::new(VecDeque::new()));
    let q2 = q1.clone();

    let worker1 = thread::spawn(move || {
        ax_println!("worker1 ...");
        for i in 0..=LOOP_NUM {
            ax_println!("worker1 [{i}]");
            q1.lock().push_back(i);
            WQ.notify_one(true);
        }
        ax_println!("worker1 ok!");
    });

    let worker2 = thread::spawn(move || {
        ax_println!("worker2 ...");
        loop {
            if let Some(num) = q2.lock().pop_front() {
                ax_println!("worker2 [{num}]");
                if num == LOOP_NUM {
                    break;
                }
                continue;
            }

            WQ.wait();
        }
        ax_println!("worker2 ok!");
    });

    ax_println!("Wait for workers to exit ...");
    let _ = worker1.join();
    let _ = worker2.join();

    ax_println!("WaitQ ok!");
}
