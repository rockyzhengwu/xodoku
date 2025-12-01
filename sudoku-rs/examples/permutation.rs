use itertools::Itertools;
fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    let k = 2;

    let combinations: Vec<Vec<i32>> = vector.into_iter().combinations(k).collect();
    println!("{:?}", combinations);
}
