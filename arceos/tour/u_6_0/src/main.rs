#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
extern crate axstd as std;
#[macro_use]
extern crate axlog;

use std::collections::VecDeque;
use std::os::arceos::modules::axsync::spin::SpinNoIrq;
use std::sync::Arc;
use std::thread;

const LOOP_NUM: usize = 256;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    ax_println!("Multi-task(Preemptible) is starting ...");

    let q1 = Arc::new(SpinNoIrq::new(VecDeque::new()));
    let q2 = q1.clone();

    let worker1 = thread::spawn(move || {
        ax_println!("worker1 ... {:?}", thread::current().id());
        for i in 0..=LOOP_NUM {
            ax_println!("worker1 [{i}]");
            q1.lock().push_back(i);
        }
        ax_println!("worker1 ok!");
    });

    let worker2 = thread::spawn(move || {
        ax_println!("worker2 ... {:?}", thread::current().id());
        loop {
            if let Some(num) = q2.lock().pop_front() {
                ax_println!("worker2 [{num}]");
                if num == LOOP_NUM {
                    break;
                }
            } else {
                ax_println!("worker2: nothing to do!");
                // TODO: it should sleep and wait for notify!
                thread::yield_now();
            }
        }
        ax_println!("worker2 ok!");
    });

    ax_println!("Wait for workers to exit ...");
    let _ = worker1.join();
    let _ = worker2.join();

    ax_println!("Multi-task(Preemptible) ok!");
}
