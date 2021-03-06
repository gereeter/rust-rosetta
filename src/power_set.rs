// Given a set, generate its power set, which is the set of all subsets of that
// set: http://rosettacode.org/wiki/Power_set

use std::vec::Vec;
use std::slice::Items;

// If set == {}
//   return {{}}
// else if set == {a} U rest
//   return power_set(rest) U ({a} U each set in power_set(rest))
fn power_set<'a, T: Clone>(items: &mut Items<'a,T>) -> Vec<Vec<T>> {
    let mut power = Vec::new();
    match items.next() {
        None       => power.push(Vec::new()),
        Some(item) => {
            for set in power_set(items).iter() {
                power.push(set.clone());
                power.push(set.clone().append_one(item.clone()));
            }
        }
    }
    power
}

#[test]
fn test() {
    let set = Vec::<int>::new();
    let power = power_set(&mut set.iter());
    assert!(power == vec!(vec!()));

    let mut set = Vec::<int>::new();
    set.push(1);
    set.push(2);
    set.push(3);
    let power = power_set(&mut set.iter());
    assert!(power == vec!(vec!(), vec!(1), vec!(2), vec!(2, 1),
                          vec!(3), vec!(3, 1), vec!(3, 2), vec!(3, 2, 1)));
}

#[cfg(not(test))]
fn main() {
    let mut set = Vec::<int>::new();
    set.push(1);
    set.push(2);
    set.push(3);
    set.push(4);
    let power = power_set(&mut set.iter());
    println!("Set      : {}", set);
    println!("Power Set: {}", power);
}
