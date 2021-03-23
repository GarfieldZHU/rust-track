use std::{collections::HashMap, usize};
use std::sync::Arc;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = Vec::new();
    let case_insensitive_input = input.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>();
    let arc_input = Arc::new(case_insensitive_input);

    for i in 0..worker_count {
        let thread_input = arc_input.clone();

        let handle = std::thread::spawn(move || {
            let mut map = HashMap::new();
            let mut pos = i;
            while pos < thread_input.len() {
                thread_input.get(pos).unwrap().chars().filter(|ch| ch.is_alphabetic()).for_each(|ch| {
                    *map.entry(ch).or_insert(0 as usize) += 1;
                });
                pos += worker_count
            }
            map
        });

        handles.push(handle);
    }

    let mut overall: HashMap<char, usize> = HashMap::new();
    for h in handles {
        for (ch, count) in h.join().unwrap() {
            *overall.entry(ch).or_insert(0 as usize) += count;
        }
    }
    overall
}