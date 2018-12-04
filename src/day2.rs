use std::collections::{HashMap, HashSet};

pub fn solve1(words: &str) -> usize {
    let mut two = 0;
    let mut three = 0;
    for line in words.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let set = map.values().collect::<HashSet<_>>();
        if set.contains(&2) {
            two += 1;
        }
        if set.contains(&3) {
            three += 1;
        }
    }
    two * three
}

fn letters_diff(word1: &str, word2: &str) -> usize {
    word1
        .chars()
        .zip(word2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .fold(0, |acc, _| acc + 1)
}

fn extract_common(word1: &str, word2: &str) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}

pub fn solve2(words: &str) -> String {
    let words = words.lines().collect::<Vec<_>>();
    for i in 0..words.len() {
        for j in (i + 1)..words.len() {
            if letters_diff(words[i], words[j]) == 1 {
                return extract_common(words[i], words[j]);
            }
        }
    }
    panic!("Could not find words with only one letter different")
}

#[test]
fn test1() {
    let test_data = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
    assert_eq!(solve1(test_data), 12);
}

#[test]
fn test2() {
    let test_data = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
    assert_eq!(&solve2(test_data), "fgij");
}
