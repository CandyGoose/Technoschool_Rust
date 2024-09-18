use std::collections::HashSet;

fn intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).cloned().collect()
}

fn main() {
    let set1: HashSet<i32> = vec![5, 22, 32, 41, 1].into_iter().collect();
    let set2: HashSet<i32> = vec![1, 46, 5, 621, 73].into_iter().collect();

    let result = intersection(&set1, &set2);

    println!("Intersection: {:?}", result);
}
