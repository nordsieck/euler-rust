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

fn is_connected((ax, ay): (uint, uint), (bx, by): (uint, uint)) -> bool {
    match ((ax as int - bx as int).abs(), (ay as int - by as int).abs()) {
        (0, 1) => true,
        (1, 0) => true,
        _ => false,
    }
}



#[test]
fn test() {
    assert_eq!(answer(), 0);
}

#[test]
fn test_render() {
    assert_eq!(render(vec![""]), vec![vec![]]);
    assert_eq!(render(vec!["1"]), vec![vec![1]]);
    assert_eq!(render(vec!["123","456"]), vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_is_connected() {
    assert_eq!(is_connected((0, 0), (1, 1)), false);
}