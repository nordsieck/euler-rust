fn answer() -> usize {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut f3: usize;
    let mut sum = 0;

    while f2 < 4000000usize {
        f3 = f2;
        f2 += f1;
        f1 = f3;
        if f2 % 2 == 0 { sum += f2 }
    }
    sum
}

#[test]
fn test() {
    if answer() != 4613732 {
        panic!("Wrong answer");
    }
}
