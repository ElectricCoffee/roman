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

fn negation_partner(val: u32) -> u32 {
    match val {
        500 | 1000 => 100,
        50  | 100  => 10,
        5   | 10   => 1,
        _          => 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
