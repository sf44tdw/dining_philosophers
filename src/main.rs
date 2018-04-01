extern crate rand;

use std::thread;
use std::sync::{Mutex, Arc};

mod table;
mod philosopher;

fn main() {
    let table = Arc::new(table::Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        philosopher::Philosopher::new("Judith Butler", 0, 1),
        philosopher::Philosopher::new("Gilles Deleuze", 1, 2),
        philosopher::Philosopher::new("Karl Marx", 2, 3),
        philosopher::Philosopher::new("Emma Goldman", 3, 4),
        philosopher::Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}