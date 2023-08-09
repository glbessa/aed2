mod hash_table;

use hash_table::HashTable;

fn main() {
    let mut h: HashTable<u8> = HashTable::with_capacity(30);
    match h.put(&String::from("gabriel"), 10) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    };

    println!("{}", h.get(&String::from("gabriel")).unwrap())
}

