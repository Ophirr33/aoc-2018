use std::collections::HashSet;

pub fn solve1(nums: &str) -> isize {
    nums.lines().map(|s| s.parse::<isize>().unwrap()).sum()
}

pub fn solve2(nums: &str) -> isize {
    let mut set = HashSet::new();
    let mut sum: isize = 0;
    set.insert(sum);
    loop {
        for n in nums.lines().map(|s| s.parse::<isize>().unwrap()) {
            sum += n;
            if set.contains(&sum) {
                return sum;
            }
            set.insert(sum);
        }
    }
}
