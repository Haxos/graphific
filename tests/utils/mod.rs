use std::fmt::Debug;

#[cfg(test)]
pub fn assert_sorted_vec_eq<T>(a: &Vec<T>, b: &Vec<T>)
where
    T: PartialEq + Ord + Clone + Debug,
{
    let mut a: Vec<T> = (*a).clone();
    let mut b: Vec<T> = (*b).clone();

    a.sort();
    b.sort();
    assert_eq!(a, b)
}
