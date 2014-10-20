fn answer() -> uint {
    let mut powers = vec![];

    for i in range(2, 21) {
        powers = merge(powers, factor(i));
    }

    let mut prod = 1u;
    for i in range(0, powers.len()) {
        prod *= pow(i + 2, powers[i]);
    }
    prod
}

fn pow(base: uint, power: uint) -> uint {
    let mut ret = 1;
    for _ in range(0, power) {
        ret *= base;
    }
    ret
}

fn factor(mut i: uint) -> Vec<uint> {
    let mut v = vec![];
    let mut n = 2;
    let mut curr = 0u;
    while i > 1 {
        if i % n == 0 {
            curr += 1;
            i /= n;
        }
        else {
            v.push(curr);
            curr = 0u;
            n += 1;
        }
    }
    v.push(curr);
    v
}

fn merge(u: Vec<uint>, v: Vec<uint>) -> Vec<uint> {
    let a;
    let b;
    if v.len() > u.len() { a = v; b = u; }
    else { a = u; b = v; }

    let mut ret = vec![];
    for i in range(0, b.len()) {
        ret.push(max(a[i], b[i]))
    }
    for i in range(b.len(), a.len()) {
        ret.push(a[i]);
    }

    ret
}

fn max(a: uint, b: uint) -> uint {
    if b > a { b }
    else { a }
}

#[test]
fn test() {
    let got = answer();
    let expected = 232792560u;
    if got != expected {
        fail!("got: {}, expected: {}", got, expected)
    }
}

#[test]
fn test_factor() {
    if factor(2) != vec![1] { fail!() }
    if factor(3) != vec![0, 1] { fail!() }
    if factor(20) != vec![2, 0, 0, 1] { fail!() }
    if factor(11) != vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1] { fail!() }
}

#[test]
fn test_merge() {
    if merge(factor(2), factor(3)) != vec![1, 1] { fail!() }
    if merge(factor(3), factor(5)) != vec![0, 1, 0, 1] { fail!() }
    if merge(factor(6), factor(8)) != vec![3, 1] { fail!() }
}