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

#[test]
fn test1() {
    let res = solve1("+1\n-2\n+5\n-3\n0\n-10");
    assert_eq!(-9, res);
}

#[test]
fn test2() {
    let res = solve2("+1\n-2\n+5\n-3\n0\n-10");
    assert_eq!(1, res);
}
