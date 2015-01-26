fn answer() -> usize {
    let mut powers = vec![];

    for i in range(2, 21) {
        powers = merge(powers, factor(i));
    }

    let mut prod = 1us;
    for i in range(0, powers.len()) {
        prod *= pow(i + 2, powers[i]);
    }
    prod
}

fn pow(base: usize, power: usize) -> usize {
    let mut ret = 1;
    for _ in range(0, power) {
        ret *= base;
    }
    ret
}

fn factor(mut i: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut n = 2;
    let mut curr = 0us;
    while i > 1 {
        if i % n == 0 {
            curr += 1;
            i /= n;
        }
        else {
            v.push(curr);
            curr = 0us;
            n += 1;
        }
    }
    v.push(curr);
    v
}

fn merge(u: Vec<usize>, v: Vec<usize>) -> Vec<usize> {
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

fn max(a: usize, b: usize) -> usize {
    if b > a { b }
    else { a }
}

#[test]
fn test() {
    let got = answer();
    let expected = 232792560us;
    if got != expected {
        panic!("got: {}, expected: {}", got, expected)
    }
}

#[test]
fn test_factor() {
    if factor(2) != vec![1] { panic!() }
    if factor(3) != vec![0, 1] { panic!() }
    if factor(20) != vec![2, 0, 0, 1] { panic!() }
    if factor(11) != vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1] { panic!() }
}

#[test]
fn test_merge() {
    if merge(factor(2), factor(3)) != vec![1, 1] { panic!() }
    if merge(factor(3), factor(5)) != vec![0, 1, 0, 1] { panic!() }
    if merge(factor(6), factor(8)) != vec![3, 1] { panic!() }
}