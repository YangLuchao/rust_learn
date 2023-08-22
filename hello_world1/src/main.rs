use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    let pair: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2: Arc<(Mutex<bool>, Condvar)> = pair.clone();

    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started: std::sync::MutexGuard<'_, bool> = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started: std::sync::MutexGuard<'_, bool> = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}