#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

// lazy static required to have static complex data types
lazy_static! {
    static ref CONV_MAP: HashMap<char, u32> = {
        let mut map = HashMap::new();
        map.insert('O', 0); // added for the sake of completion
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        map
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
