fn main() {
    let nums = vec![1, 2, 3, 4];
    let upper_limit = 2;

    let res = nums
        .iter()
        .filter(|num| **num < upper_limit)
        .map(|num| num + 1)
        .collect::<Vec<u64>>();

    println!("{:?}", res);
}
