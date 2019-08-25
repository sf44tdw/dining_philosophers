
use rand::{thread_rng, Rng};

use std::thread;
use std::time::Duration;

use table::Table;

pub struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
   pub fn new(name: &str, p_left: usize, p_right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: p_left,
            right: p_right,
        }
    }

   pub fn eat(&self, table: &Table) {
        let _left: std::sync::MutexGuard<'_, ()> = table.forks[self.left].lock().unwrap();

        let taking_time: u64 = thread_rng().gen_range(1, 150);

        println!("{} is taking. {} ms.", self.name, taking_time);

        thread::sleep(Duration::from_millis(taking_time));
        let _right: std::sync::MutexGuard<'_, ()> = table.forks[self.right].lock().unwrap();

        let eating_time: u64 = thread_rng().gen_range(1, 1000);

        println!("{} is start eating. {} ms.", self.name, eating_time);

        thread::sleep(Duration::from_millis(eating_time));

        println!("{} is done eating.", self.name);
    }
}