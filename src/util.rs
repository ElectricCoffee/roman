/// converts a roman digit into a latin digit, not taking negation rules into account
pub fn conv_digit(digit: char) -> Option<u32> {
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

/// turns an unsigned 32-bit integer into a vector of single digits
pub fn digitize(n: u32) -> Vec<u32> {
    fn helper(n: u32, v: &mut Vec<u32>) {
        if n >= 10 {
            helper(n / 10, v);
        }
        v.push(n % 10);
    }
    let mut vec = Vec::new();
    helper(n, &mut vec);
    vec
}
