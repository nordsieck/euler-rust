fn answer() -> usize {
    let mut sum = 0;

    for i in 0..1000 {
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