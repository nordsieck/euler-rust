fn answer() -> usize {
    let mut sum = 0u;

    for i in range(0u, 1000) {
        if i % 3 == 0 { sum += i; }
        else if i % 5 == 0 { sum += i; }
    }

    sum
}

#[test]
fn test() {
    if answer() != 233168 {
        panic!("Wrong answer");
    }
}