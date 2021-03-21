use std::{collections::HashMap};
use std::sync::{Arc, Mutex, RwLock};

pub fn frequency(input: &'static[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = Vec::new();
    let shared_map = Arc::new(RwLock::new(HashMap::new()));
    let shared_iters = Arc::new(Mutex::new(input.iter()));

    for _i in 0..worker_count {
        let thread_map = Arc::clone(&shared_map);
        let thread_iter = shared_iters.clone();

        let handle = std::thread::spawn(move || {
            while let Some(value) = thread_iter.lock().unwrap().next() {
               value.chars().for_each(|ch| {
                   let mut map = thread_map.read().unwrap();

                    *map.entry(ch).or_insert(1) += 1;
               })
            }

        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    shared_map.as_ref()
}
