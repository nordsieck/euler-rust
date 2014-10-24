fn answer() -> uint {
    0
}

fn render(s: &str) -> Vec<Vec<uint>> {
    vec![vec![0]]
}

#[test]
fn test() {
    if answer() != 0 { fail!() }
}

#[test]
fn test_render() {
    assert_eq!(render(""), vec![vec![0]]);
}