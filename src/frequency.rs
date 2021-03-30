extern crate crossbeam;
use std::{collections::{HashMap, HashSet}, sync::Mutex};

pub fn get_frequency(total_array: HashSet<String>) -> Vec<(&&str, &u32)> {
    let mut frequency: Mutex<HashMap<&str, u32>> = Mutex::new(HashMap::new());
    
    crossbeam::scope(|scope| {
        for x in 0..1 {
            scope.spawn(|x| {
                for sentence in &total_array {
                    for word in sentence.split_whitespace() {
                        let mut frequency = frequency.lock().unwrap();
                        *frequency.entry(word).or_insert(0) += 1;

                    }
                }
            });
        }
    });

    let mut count_vec: Vec<(&&str, &u32)> = frequency.get_mut().unwrap().iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1))
    
}