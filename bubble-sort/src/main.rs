use rand::Rng;
use std::time::Instant;

const SIZE: usize = 10_000;

mod bubble_sort;

pub fn generate_nums(size: &usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let mut nums = Vec::with_capacity(*size);

    for _ in 0..SIZE {
        nums.push(rng.gen_range(0..*size));
    }

    nums
}

#[allow(dead_code)]
fn print_vec(vec: &Vec<usize>) {
    println!("{:?}", vec);
}

fn is_vec_sorted(vec: &Vec<usize>) -> bool {
    let mut i = 0;

    loop {
        if i == vec.len() - 1 {
            break true;
        }

        if vec[i] < vec[i + 1] {
            break false;
        }

        i += 1;
    }
}

#[allow(dead_code)]
fn main() {
    let start = Instant::now();

    let mut nums = generate_nums(&SIZE);

    bubble_sort::bubble_sort(&mut nums);

    assert!(is_vec_sorted(&nums));

    println!("Total: {:.2?}", start.elapsed());
}
