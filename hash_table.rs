use std::fmt;
use std::error::{Error};
use std::convert::TryFrom;

/*
pub struct HashTable<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    hash: fn(K, usize) -> Result<usize, String>,
}*/

pub struct HashTable<V> {
    keys: Vec<String>,
    values: Vec<V>,
    hash: fn(&String, usize) -> Result<usize, String>,
}

//impl<String, V> HashTable<String, V> {
impl<V> HashTable<V> {
    pub fn new() -> Self {
        let mut keys: Vec<String> = Vec::with_capacity(20);
        //keys.set_len(20);
        HashTable {
            keys: Vec::with_capacity(20),
            values: Vec::with_capacity(20),
            hash: string_hashing,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashTable {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            hash: string_hashing,
        }
    }

    pub fn get(&mut self, key: &String) -> Option<&V> {
        let pos = match (self.hash)(key, self.keys.capacity()) {
            Ok(p) => p,
            Err(_) => return None,
        };

        self.values.get(pos)
    }

    pub fn put(&mut self, key: &String, value: V) -> Result<(), String> {
        let pos = (self.hash)(key, self.keys.capacity())?;

        match self.keys.get(pos) {
            Some(_) => {
                self.keys.insert(pos, key.clone());
                self.values.insert(pos, value);
            },
            None => {
                self.keys.insert(pos, key.clone());
                self.values.insert(pos, value);
            }
        }

        Ok(())
    }
}

pub fn universal_hashing<T>(key: T, capacity: usize) -> Result<usize, String> {
    todo!();
}

pub fn division_hashing(key: usize, capacity: usize) -> Result<usize, String> {
    Ok(key % capacity)
}

pub fn multiplication_hashing(key: usize, capacity: usize) -> Result<usize, String> {
    let a: f32 = 0.6180339887;
    let mut val: f32 = (key as f32) * a;
    val -= val.trunc();
    
    Ok((val.trunc() as usize) * capacity)
}

pub fn fold_hashing(key: usize, capacity: usize) -> Result<usize, String> {
    let s1 = key >> 10;
    let s2 = match capacity.checked_sub(1) {
        Some(num) => num & key,
        None => return Err(String::from("Capacity equals to 0!"))
    };

    Ok(s1 ^ s2)
}

fn string_hashing(key: &String, capacity: usize) -> Result<usize, String> {
    let mut pos: usize = 7;

    for c in key.chars() {
        pos = 31 * pos;
        pos += match usize::try_from(c as u32) {
            Ok(num) => num,
            Err(err) => return Err(err.to_string()),
        };
    }

    Ok(pos % capacity)
}
