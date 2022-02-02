use std::thread::{self, JoinHandle};

fn main() {
    let cpus = num_cpus::get();
    let mut handles = Vec::with_capacity(cpus);
    
    for i in 1..cpus + 1 {
        handles.push(worker(i as f64, cpus));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn worker(mut i: f64, total: usize) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let n = i * ((10_f64.powf(i.log10().floor() + 1.0)) + 1.0);

            if n.sqrt() % 1.0 == 0.0 {
                println!("{}", i);
            }

            i += total as f64;
        }
    })
}