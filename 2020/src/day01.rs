use std::collections::HashSet;

pub fn part1(input: String) {
    let nums: Vec<_> = input.lines().map(|line: &str| line.parse::<i32>().unwrap()).collect();
    let (x, y) = two_sum(nums, 2020);
    println!("{}", x * y);
}

fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut seen = HashSet::new();
    for num in nums {
        if seen.contains(&(target - num)) {
            return (target - num, num);
        }
        seen.insert(num);
    }

    panic!("No solution found");
}

pub fn part2(input: String) {
    let nums: Vec<_> = input.lines().map(|line: &str| line.parse::<i32>().unwrap()).collect();
    let (x, y, z) = three_sum(nums, 2020);
    println!("{}", x * y * z);
}

fn three_sum(nums: Vec<i32>, target: i32) -> (i32, i32, i32) {
    let available: HashSet<_> = nums.iter().cloned().collect();
    assert_eq!(nums.len(), available.len(), "Assumption: there are no duplicates");
    for (xi, x) in nums.iter().cloned().enumerate() {
    for y in nums[xi..].iter().cloned() {
        let z = target - (x + y);
        if x == z || y == z {
            continue;
        }
        if available.contains(&z) {
            return (x, y, z)
        }
    }
    }

    panic!("No solution found");
}