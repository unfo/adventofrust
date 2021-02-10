
// fn main() {
//     let data: Vec<_> = vec![1, 12, 15, 1, 12, 3, 5].into_iter().unique().collect();
//     println!("{:?}", data);
// }

pub mod prelude;
pub mod point2d;
pub mod vecneighbor;

use std::collections::{hash_map::DefaultHasher, HashSet};
use std::option::Option;
use std::hash::{Hash, Hasher};

pub struct Unique<I> {
    iter: I,
    seen: HashSet<u64>,
}

fn hash_once<T>(item: &T) -> u64
    where T: Hash
{
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

impl<I> Iterator for Unique<I>
    where I: Iterator,
          I::Item: Hash + Sized
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let seen = &mut self.seen;
        match self.iter.find(|x| !seen.contains(&hash_once(x))) {
            None => None,
            Some(item) => {
                seen.insert(hash_once(&item));
                Some(item)
            }
        }
    }
}

pub trait UniqueAdapter: Iterator {
    fn unique(self) -> Unique<Self>
        where Self: Sized,
              Self::Item: Hash
    {
        Unique {
            iter: self,
            seen: HashSet::new(),
        }
    }
}

impl<I> UniqueAdapter
    for I where I: Iterator
{
}

#[test]
fn foo(){
    let ss: Vec<_> = vec!["test", "two", "two", "three"];
    let si: Vec<_> = ss.into_iter()
        .unique()
        .collect();
    println!("{:?}", si);
}


pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}
