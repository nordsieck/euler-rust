fn answer() -> usize { 
    let mut num = 600851475143us;
    let mut gpf = 2us; // greatest prime factor

    while num > 1 {
        if num % gpf == 0 { num /= gpf }
        else { gpf += 1 }
    }
    println!("gpf: {}", gpf);

    gpf
}

#[test]
fn test() {
    if answer() != 6857us {
        panic!("Wrong answer");
    }
}