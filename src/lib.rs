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

fn convert_vec(vec: Vec<u32>) -> Option<u32> {
    let mut last = 0;
    let mut acc = 0;
    for current in vec  {
        // if the last value is less than the current value, attempt subtraction
        if last < current && last != 0 {
            let partner = negation_partner(current);
            // if the last value is valid, subtract then increment
            // making sure to reset the last value to 0
            if partner == last {
                // remove the last added value
                acc -= last;
                // then add the last value subtracted from the current
                acc += current - last;
                last = 0;
            } else {
                return None;
            }
        // otherwise, if the last value is greater, or 0, increment
        // making sure to bump the last value to the current one
        } else {
            acc += current;
            last = current;
        }
    }
    Some(acc)
}

pub fn from_roman(num_str: &str) -> Option<u32> {
    let mut result = Vec::new();
    for (_, c) in num_str.chars().enumerate() {
        if let Some(num) = conv_digit(c) {
            result.push(num);
        } else {
            return None;
        }
    }
    convert_vec(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn invalid_string() {
        // invalid characters
        let res = super::from_roman("SPQR");
        assert_eq!(res, None);

        // valid + invalid characters
        let res = super::from_roman("XVQR");
        assert_eq!(res, None);

        // valid characters in invalid order
        let res = super::from_roman("CMLCXLVXILI");
        assert_eq!(res, None);
    }

    #[test]
    fn valid_string() {
        let res = super::from_roman("XVII");
        assert_eq!(res, Some(17));

        let res = super::from_roman("IX");
        assert_eq!(res, Some(9));

        let res = super::from_roman("XL");
        assert_eq!(res, Some(40));

        let res = super::from_roman("XLIX");
        assert_eq!(res, Some(49));

        let res = super::from_roman("MCCXLVI");
        assert_eq!(res, Some(1246));

        let res = super::from_roman("CMXCIX");
        assert_eq!(res, Some(999));
    }
}
