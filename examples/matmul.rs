use rusty_math::set::*;
use rusty_math::axiom::*;
use typenum::*;

fn main() {
    let a : BasicVector<u32, U3> = BasicVector::from_vec(vec![1, 2, 3]);
    let b : BasicMatrix<u32, U3, U3> = BasicMatrix::from_vec(vec![
        1, 0, 0,
        0, 1, 0, 
        0, 0, 1,
    ]);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", b * a);
}