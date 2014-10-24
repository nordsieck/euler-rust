fn answer() -> uint {
    0
}

fn render(s: Vec<&str>) -> Vec<Vec<uint>> {
    let mut v = vec![];
    for i in range(0, s.len()) {
        let mut w = vec![];
        for j in range(0, s[0].len()) {
            w.push(s[i].char_at(j) as uint - '0' as uint);
        }
        v.push(w);
    }
    v
}

#[test]
fn test() {
    if answer() != 0 { fail!() }
}

#[test]
fn test_render() {
    assert_eq!(render(vec![""]), vec![vec![]]);
    assert_eq!(render(vec!["1"]), vec![vec![1]]);
    assert_eq!(render(vec!["123","456"]), vec![vec![1, 2, 3], vec![4, 5, 6]]);
}