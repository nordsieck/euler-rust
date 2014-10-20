fn answer() -> uint {
    let mut i = 0u;
    let mut candidate = 0u;
    loop {
        let c = row(i);
        if c > candidate { candidate = c; }
        if candidate > row(i + 1) { break; }
        i += 1;
    }
    candidate
}

// difference from 999 -> largest palindrome, largest product
fn row(i: uint) -> (uint) {
    let mut x = 999 - i;
    let mut y = 999;
    let mut prod;

    while x <= 999 && y >= 1 {
        prod = x * y;
        if palindrome(prod) { return prod }

        x += 1;
        y -= 1;
    }
    0
}

fn max(i: uint) -> uint {
    let j = i / 2;
    (999 - i + j) * (999 - j)
}

fn palindrome(i: uint) -> bool {
    let mut rev = digits(i);
    rev.reverse();
    rev == digits(i)
}

fn digits(mut i: uint) -> Vec<uint> {
    let mut v = vec![];
    while i != 0 {
        v.push(i % 10);
        i /= 10;
    }
    v
}

#[test]
fn test() {
    let got = answer();
    let expected = 906609u;
    if got != expected {
        fail!("got: {}, expected: {}", got, expected);
    }
}

#[test]
fn test_digits() {
    if digits(1) != vec![1u] { fail!() }
    if digits(11) != vec![1u, 1u] { fail!() }
    if digits(1234) != vec![4u, 3u, 2u, 1u] { fail!() }
}

#[test]
fn test_palindrome() {
    if !palindrome(1) { fail!() }
    if !palindrome(66) { fail!() }
    if !palindrome(206602) { fail!() }
    if !palindrome(2006002) { fail!() }

    if palindrome(67) { fail!() }
    if palindrome(206702) { fail!() }
}

#[test]
fn test_row() {
    if row(0) != (0) { fail!() }
    if row(998) != (999) { fail!() }
}

#[test]
fn test_max() {
    if max(0) != 998001 { fail!() }
}