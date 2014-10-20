fn answer() -> uint {
    primes(run_sieve(sieve(110000)))[10000]
}

fn sieve(n: uint) -> Vec<bool> {
    let mut v = Vec::with_capacity(n);
    v.push(false);
    v.push(false);
    for _ in range(2, n) {
        v.push(true);
    }
    v
}

fn run_sieve(mut v: Vec<bool>) -> Vec<bool> {
    for i in range (2, v.len()) {
        if v[i] {
            for j in range(2, v.len() / i) {
                *v.get_mut(i * j) = false;
            }
        }
    }
    v
}

fn primes(v: Vec<bool>) -> Vec<uint> {
    let mut u = vec![];
    for i in range(0, v.len()) {
        if v[i] { u.push(i); }
    }
    u
}

#[test]
fn test() {
    assert_eq!(answer(), 104743u);
}

#[test]
fn test_sieve() {
    assert_eq!(sieve(6), vec![false, false, true, true, true, true]);
}

#[test]
fn test_run_sieve() {
    assert_eq!(run_sieve(sieve(6)), vec![false, false, true, true, false, true]);
    assert_eq!(run_sieve(sieve(12)), vec![false, false, true, true, false, true, false, true, false, false, false, true]);
}

#[test]
fn test_primes() {
    assert_eq!(primes(run_sieve(sieve(6))), vec![2, 3, 5]);
    assert_eq!(primes(run_sieve(sieve(12))), vec![2, 3, 5, 7, 11]);
}