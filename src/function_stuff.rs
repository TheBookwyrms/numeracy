use crate::{functions_and_math::{common_functions::*, traits::MathValue}, matrices, matrices2::Matrix};

fn lines() {
    println!();
    println!();
    println!();
}

pub fn functions() {
    let a = 5.as_math_tree();
    let b = number(6);
    let c = number(7);
    let d = number(8);
    let e = number(9);
    let f = variable('a');
    let g = variable('b');
    let h = variable('c');

    let tests = [
        add(a.clone(), b.clone()),
        add(a.clone(), add(b.clone(), c.clone())),
        a.clone()+b.clone()+c.clone(),
        cos(a.clone()),
        cos(sin(pow(a.clone(), b.clone()))+c.clone()-d.clone()),
        a.clone()+f.clone(),
        atan(c*f.clone() + a*b),
        f.clone()+g.clone(),
        g.clone()*(f.clone()/h.clone()),
        g.clone()*g.clone()+h.clone()-f.clone()-h+f,
        g.clone()+g,
    ];

    for test in tests {
        println!("{}, {:?}, {:?}", test, test.get_variables(), test.get_variable_nature())
    }


    lines();

    let mat = matrices::Matrix::from_2darray([[number(3), number(2)]]).as_math_tree();
    let mat = Matrix::from_1darray([number(3), number(2)]).as_math_item();
}