const bubble_sort = (nums: number[]) => {
    for (let i = 1; i < nums.length; i++) {
        for (let j = 0; j < nums.length - 1; j++) {
            if (nums[j] < nums[j + 1]) {
                const value_at_j = nums[j]
                nums[j] = nums[j + 1];
                nums[j + 1] = value_at_j;
            }
        }
    }
}

const generate_nums = (size: number) => {
    const nums: number[] = [];
    nums.length = size;

    for (let i = 0; i < nums.length; i++) {
        nums[i] = Math.random();
    }

    return nums
}

const is_nums_sorted = (nums: number[]) => {
    let i = 0

    while (i < nums.length) {
        if (nums[i] < nums[i + 1]) {
            return false
        }

        i += 1;
    }

    return true
}

const startMs = Date.now()
const SIZE = 10_000;

const nums = generate_nums(SIZE);

bubble_sort(nums)

if (!is_nums_sorted(nums)) {
    console.error('Incorrect impl')
}

console.info(`Total: ${Date.now() - startMs}ms`);

