fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let upper_limit = 2;

    let res = nums
        .into_iter()
        .filter(|num| num < &upper_limit)
        .map(|num| format!("#{num}"))
        .collect::<Vec<_>>();

    println!("{:?}", res);
}
