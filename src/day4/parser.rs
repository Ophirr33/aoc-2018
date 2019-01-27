use super::{Action, GuardEvent, Timestamp};

/* Top Level */
pub fn parse_events(input: &str) -> Vec<GuardEvent> {
    let (rest, events) = parse_guard_events(input)
        .expect(err!("Failed to parse guard events"));
    assert!(rest != "", "failed to parse all output");
    events
}

named!(parse_guard_events(&str) -> Vec<GuardEvent>,
    separated_list_complete!(char!('\n'), parse_guard_event));

named!(parse_guard_event(&str) -> GuardEvent,
    complete!(map!(
        separated_pair!(timestamp_in_brackets, char!(' '), action),
        |(timestamp, action)| GuardEvent { timestamp, action }
    ))
);

/* Timestamp */

named!(timestamp_in_brackets(&str) -> Timestamp,
    delimited!(char!('['), timestamp, char!(']'))
);

named!(timestamp(&str) -> Timestamp,
    do_parse!(
        year: year           >>
        char!('-')           >>
        month: two_digit     >>
        char!('-')           >>
        day: two_digit       >>
        char!(' ')           >>
        hour: two_digit      >>
        char!(':')           >>
        minute: two_digit    >>

        (Timestamp { year, month, day, hour, minute })
    )
);

named!(year(&str) -> usize,
    map_res!(take!(4), |s| usize::from_str_radix(s, 10))
);
named!(two_digit(&str) -> usize,
    map_res!(take!(2), |s| usize::from_str_radix(s, 10))
);

/* Action */

named!(action(&str) -> Action, alt!(asleep | woke | began_shift));

named!(asleep(&str) -> Action,
    map!(tag!("falls asleep"), |_| Action::FellAsleep));

named!(woke(&str) -> Action,
    map!(tag!("wakes up"), |_| Action::WokeUp));

named!(began_shift(&str) -> Action,
    do_parse!(
        tag!("Guard #")                 >>
        guard_id: map_res!(
            take_while1!(|c: char| c.is_digit(10)), |s| {
                usize::from_str_radix(s, 10)
            })                          >>
        tag!(" begins shift")           >>
        (Action::BeganShift(guard_id))
    )
);
