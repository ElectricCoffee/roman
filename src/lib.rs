#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

fn conv_digit(digit: char) -> Option<u32> {
    match digit {
        'O' => Some(0),
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _   => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
