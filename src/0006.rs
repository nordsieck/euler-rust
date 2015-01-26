fn answer() -> usize {
    square_of_sums(101) - sum_of_squares(101)
}

fn sum_of_squares(i: usize) -> usize {
    let mut ret = 0;
    for j in range(1, i) {
        ret += j * j
    }
    ret
}

fn square_of_sums(i: usize) -> usize {
    let mut ret = 0;
    for j in range(1, i) {
        ret += j;
    }
    ret * ret
}

#[test]
fn test() {
    if answer() != 25164150us { panic!("wrong answer"); }
}

#[test]
fn test_sum_of_squares() {
    if sum_of_squares(2) != 1 { panic!() }
    if sum_of_squares(3) != 5 { panic!() }
    if sum_of_squares(4) != 14 { panic!() }
}

#[test]
fn test_square_of_sums() {
    if square_of_sums(2) != 1 { panic!() }
    if square_of_sums(3) != 9 { panic!() }
    if square_of_sums(4) != 36 { panic!() }
}