use numeracy::general_math::cartesian_product;
use numeracy::general_math::comparisons::unordered_equality;


#[test]
fn cartesian_product_two_vec() {
    let a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];

    let cp = cartesian_product::cartesian_product([a1, a2]);

    let unord_eq = unordered_equality(cp, vec![
        vec![1, 4],
        vec![1, 5],
        vec![1, 6],
        vec![2, 4],
        vec![2, 5],
        vec![2, 6],
        vec![3, 4],
        vec![3, 5],
        vec![3, 6],
    ]);

    assert!(unord_eq);

    //assert_eq!(cp, vec![
    //    vec![1, 4],
    //    vec![1, 5],
    //    vec![1, 6],
    //    vec![2, 4],
    //    vec![2, 5],
    //    vec![2, 6],
    //    vec![3, 4],
    //    vec![3, 5],
    //    vec![3, 6],
    //]);
}
#[test]
fn cartesian_product_three_vec() {
    let a1 = vec![1, 2];
    let a2 = vec![3, 4];
    let a3 = vec![5, 6];

    let cp = cartesian_product::cartesian_product([a1, a2, a3]);

    let unord_eq = unordered_equality(cp, vec![
        vec![1, 3, 5],
        vec![1, 3, 6],
        vec![1, 4, 5],
        vec![1, 4, 6],
        vec![2, 3, 5],
        vec![2, 3, 6],
        vec![2, 4, 5],
        vec![2, 4, 6],
    ]);

    assert!(unord_eq);

    //assert_eq!(cp, vec![
    //    vec![1, 3, 5],
    //    vec![1, 3, 6],
    //    vec![1, 4, 5],
    //    vec![1, 4, 6],
    //    vec![2, 3, 5],
    //    vec![2, 3, 6],
    //    vec![2, 4, 5],
    //    vec![2, 4, 6],
    //]);
}