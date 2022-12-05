fn main() {
    let t = (1, 1.8, 'c');
    assert_eq!(t.0, 1);
    assert_eq!(t.1, 1.8);
    assert_eq!(t.2, 'c');

    let(a, b, c) = t;

    assert_eq!(a, 1);
    assert_eq!(b, 1.8);
    assert_eq!(c, 'c');

    let a = [1, 2, 3, 4, 5];
    assert_eq!(a[0], 1);

    let a = [3; 5];
    assert_eq!(a, [3, 3, 3, 3, 3]);
}
