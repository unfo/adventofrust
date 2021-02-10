#![allow(unused_macros)]
#![allow(unused_imports)]

use itertools::Itertools;

// ANCHOR: vecneighbors
pub trait VecNeighbors<T> {
    fn neighbors(&self) -> Vec<Vec<T>>;
}

// ANCHOR: vecneighbors_macro
macro_rules! VecNeighbor_for {
    ($vectype:ty) => {
        impl VecNeighbors<$vectype> for Vec<$vectype> {
            fn neighbors(&self) -> Vec<Vec<$vectype>> {
                self
                    .iter()
                    .map(|v|(v-1 ..= v+1).into_iter())
                    .multi_cartesian_product()
                    .filter(|x| x != self)
                    .collect::<Vec<_>>()
            }
        }
    };
}
// ANCHOR_END: vecneighbors_macro
// ANCHOR_END: vecneighbors

VecNeighbor_for!(i8);
VecNeighbor_for!(i16);
VecNeighbor_for!(i32);
VecNeighbor_for!(i64);
VecNeighbor_for!(i128);
VecNeighbor_for!(isize);

#[cfg(test)]
mod test {
    use super::*;
/*
// ANCHOR: vecneighbors_impl
    VecNeighbor_for!(i32);
// ANCHOR_END: vecneighbors_impl
*/
#[test]
fn test_foo() {
    use itertools::Itertools;
    let v:Vec<Vec<_>> = vec![vec![1,2],vec![3,4],vec![5,6]];
    let vp:Vec<Vec<_>> = v.into_iter()
        .multi_cartesian_product()
        .collect();
    println!("{:?}", &vp);
}


    #[test]
// ANCHOR: vecneighbors_test
    fn test_vecneighbor() {

        let _coord:Vec<i32> = vec![0,0];
        for x in _coord.neighbors().into_iter() {
            println!("{:?}", &x);
        }
    }
// ANCHOR_END: vecneighbors_test
}

// ----- Coordinates
// #[derive(Debug, Default, Clone, PartialEq, Eq)]
// pub struct Coordinate<T> (Vec<T>);

// impl<T> Coordinate<T> {
//     pub fn new(v: Vec<T>) -> Coordinate<T> {
//         Coordinate::<T>(v)
//     }
//     pub fn dims(&self) -> usize {
//         self.0.len()
//     }
// }

// impl Coordinate<i32> {
//     pub fn neighbors(&self) -> impl Iterator<Item = Coordinate<i32>> {
//         self.0
//             .iter()
//             .map(|v|(v-(1 as i32) .. v+(1 as i32)).into_iter())
//             .multi_cartesian_product()
//             .map(|z| Coordinate::new(z))
//     }    
// }


// #[test]
// fn test_cvec_i8_neigh() {
//     let _coord:Vec<i8> = vec![1,2,3];
//     for x in _coord.neighbors().into_iter() {
//         println!("{:?}", &x);
//     }
// }
// #[test]
// fn test_coordinate() {
//     let _coord:Coordinate<i32> = Coordinate::new(vec![1,2,3]);
// }
// #[test]
// fn test_coordinate_i32_neigh() {
//     let _coord:Coordinate<i32> = Coordinate::new(vec![1,2,3]);
//     for x in _coord.neighbors().into_iter() {
//         println!("{:?}", &x);
//     }
// }

// #[test]
// fn test_dundun() {
//     let a = vec![1,2,3];
//     // let b:Vec<_> = .collect();
    
//     let c = a.iter().map(|v|(v-1..v+1)).map(|x|x.into_iter()).multi_cartesian_product();
//     // let bi = b.into_iter();
//     // let bif = bi.next().unwrap();
//     // let bis = bi.next().unwrap();
//     // let cp = bif.cartesian_product(bis).collect();
//     // let mut c = bi.fold(bif.clone(), |acc,i| acc.cartesian_product(i.clone()));
//     for x in c {
//         println!("{:?}", &x);
//     }
//     dbg!(&a);
// }