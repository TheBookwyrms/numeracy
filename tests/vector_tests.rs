use numeracy::vectors::vector::Vector;

#[test]
fn dot1() {
    let v1 = Vector::from_1darray([1, 2, 3, 4]);
    let v2 = Vector::from_1darray([5, 6, 7, 8]);
    let b = v1.dot(&v2).unwrap();
    assert_eq!(b, 5+12+21+32)
}