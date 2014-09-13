fn answer() -> uint {
    let mut f1 = 0u;
    let mut f2 = 1u;
    let mut f3: uint;
    let mut sum = 0u;

    while f2 < 4000000u {
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
        fail!("Wrong answer");
    }
}
