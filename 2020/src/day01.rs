use std::collections::HashSet;

pub fn part1(input: String) {
    let nums: Vec<_> = input.lines().map(|line: &str| line.parse::<i32>().unwrap()).collect();
    let (x, y) = two_sum(nums, 2020);
    println!("{}", x * y);
}

fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut seen = HashSet::new();
    for num in nums {
        if seen.contains(&num) {
            return (target - num, num);
        }
        seen.insert(target - num);
    }

    panic!("No solution found");
}
