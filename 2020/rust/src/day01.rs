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

#[allow(dead_code)]
pub fn partk(input: String) {
    let nums: Vec<_> = input.lines().map(|line: &str| line.parse::<i32>().unwrap()).collect();
    let kset = k_sum(3, nums, 2020);
    println!("{}", kset.iter().fold(1, |a, b| a * b));
}

fn k_sets(k: i32, nums: &Vec<i32>) -> Vec<HashSet<i32>> {
    if k == 1 {
        return nums.iter().map(|&x| [x].iter().cloned().collect()).collect();
    }

    let smalls = k_sets(k - 1, nums);
    let mut result: Vec<_> = Vec::new();

    for num in nums.iter().cloned() {
        for small in &smalls {
            let mut item: HashSet<i32> = small.iter().cloned().collect();
            if item.insert(num) {
                result.push(item);
            }
        }
    }

    return result;
}

fn k_sum(k: i32, nums: Vec<i32>, target: i32) -> Vec<i32> {
    // make all k-1 sets from nums
    let lhss = k_sets(k - 1, &nums);

    // check if each sets sum + z == target
    let available: HashSet<_> = nums.iter().cloned().collect();
    for lhs in lhss {
        let y: i32 = lhs.iter().sum();
        let z = target - y;
        if !lhs.contains(&z) && available.contains(&z) {
            let mut result = lhs;
            result.insert(z);
            return result.iter().cloned().collect();
        }
    }
    
    panic!("No solution found");
}