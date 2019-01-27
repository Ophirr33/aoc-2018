use crate::err;
use std::collections::HashMap;

mod parser;
use parser::parse_events;

pub fn solve1(input: &str) -> usize {
    let mut events = parse_events(input);
    events.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));
    let mut iter = events.into_iter();
    let mut max = HashMap::new();
    let mut id = if let Some(Action::BeganShift(n)) = iter.next().map(|e| e.action) {
        n
    } else {
        panic!(err!("should start with a BeginShift"));
    };

    loop {
        match iter.next() {
            Some(GuardEvent {
                action: Action::BeganShift(n),
                timestamp: _,
            }) => id = n,
            Some(GuardEvent {
                action: Action::FellAsleep,
                timestamp: timestamp_start,
            }) => match iter.next() {
                Some(GuardEvent {
                    action: Action::WokeUp,
                    timestamp: timestamp_end,
                }) => insert_into(&mut max, id, timestamp_start.minute, timestamp_end.minute),
                _ => panic!(err!("Bleh, woke not followed by asleep")),
            },
            Some(GuardEvent {
                action: Action::WokeUp,
                timestamp: _
            }) => panic!("Bleh, asleep before woke"),
            None => {
                break;
            },
        }
    }

    let (id, mins) = max
        .into_iter()
        .max_by_key(|t: &(usize, [usize; 60])| {
            let (id, mins) = t;
            mins.iter().sum()
        }).unwrap();
    mins.iter().enumerate().max_by_key(|(_, value)| value).map(|(idx, _)| idx).unwrap()
}

fn insert_into(count: &mut HashMap<usize, [usize; 60]>, id: usize, start: usize, end: usize) {
    let minutes = count.entry(id).or_insert([0; 60]);
    (start..end).for_each(|i| minutes[i] += 1);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

#[derive(Debug)]
pub enum Action {
    BeganShift(usize),
    FellAsleep,
    WokeUp,
}

#[derive(Debug)]
pub struct GuardEvent {
    timestamp: Timestamp,
    action: Action,
}

#[test]
fn test1() {
    let test_data = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
    assert_eq!(solve1(test_data), 240)
}
