use std::{collections::{HashMap, hash_map::Entry}};
use aoc_rs::{get_input, parser::Parser};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SleepWake {
    Sleep,
    Wake
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Guard(i64);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct LogLine {
    guard: Guard,
    state: SleepWake,
    minute: i64,
}

fn main() -> anyhow::Result<()> {
    let input = get_input(2018, 4)?;
    let log = parse(&input);

    println!("part1: {}", part1(&log));
    println!("part2: {}", part2(&log));

    Ok(())
}

fn parse(input: &str) -> Vec<LogLine> {
    let lines = input.trim().split('\n').sorted();
    let mut guard = Guard(-1);
    let mut log = Vec::new();

    for line in lines {
        let mut parser = Parser::new(line);
        parser.eat_str("[1518-");
        let _month = parser.i64();
        parser.eat('-');
        let _day = parser.i64();
        parser.eat(' ');
        let _hour = parser.i64();
        parser.eat(':');
        let minute = parser.i64();
        parser.eat(']');

        if line.contains("Guard") {
            parser.eat_str(" Guard #");
            guard = Guard(parser.i64());
        } else if line.contains("falls") {
            log.push(LogLine { guard, minute, state: SleepWake::Sleep });
        } else if line.contains("wakes") {
            log.push(LogLine { guard, minute, state: SleepWake::Wake });
        } else {
            assert!(false);
        }
    }

    log
}

fn part1(log: &[LogLine]) -> i64 {
    let mut sleep_amounts = HashMap::new();

    let mut sleep_start = 0;

    for line in log {
        if line.state == SleepWake::Sleep {
            sleep_start = line.minute;
            continue
        }
        let sleep_time = line.minute - sleep_start;

        match sleep_amounts.entry(line.guard) {
            Entry::Occupied(o) => { *o.into_mut() += sleep_time; }
            Entry::Vacant(v) => { v.insert(sleep_time); }
        }
    }

    let sleepiest_guard = *sleep_amounts
        .iter()
        .max_by_key(|(_k, &v)| v)
        .unwrap()
        .0;

    let mut sleep_minutes = [0; 60];

    for line in log {
        if line.guard != sleepiest_guard {
            continue
        }

        if line.state == SleepWake::Sleep {
            sleep_start = line.minute;
            continue
        }

        for index in (sleep_start as usize)..(line.minute as usize) {
            sleep_minutes[index] += 1;
        }
    }

    let sleepiest_minute = sleep_minutes
        .iter()
        .enumerate()
        .max_by_key(|(_i, &v)| v)
        .unwrap()
        .0 as i64;

    sleepiest_guard.0 * sleepiest_minute
}

fn part2(log: &[LogLine]) -> i64 {
    let mut guard_sleep_minutes = HashMap::new();
    let mut sleep_start = 0;

    for line in log {
        if line.state == SleepWake::Sleep {
            sleep_start = line.minute;
            continue;
        }

        let sleep_minutes_entry = guard_sleep_minutes.entry(line.guard);
        if let Entry::Vacant(v) = sleep_minutes_entry {
            v.insert([0; 60]);
        }
        let sleep_minutes;
        if let Entry::Occupied(sleep_minutes_entry) = guard_sleep_minutes.entry(line.guard) {
            sleep_minutes = sleep_minutes_entry.into_mut();
        } else {
            todo!()
        }

        for index in (sleep_start as usize)..(line.minute as usize) {
            sleep_minutes[index] += 1;
        }
    }

    let mut guard_sleepiest_minutes = HashMap::new();
    for (guard, minutes) in guard_sleep_minutes {
        let sleepiest_minute = minutes.iter().enumerate().max_by_key(|(_k, &v)| v).unwrap();
        guard_sleepiest_minutes.insert(guard, (sleepiest_minute.0, *sleepiest_minute.1));
    }

    let sleepiest_guard_minute = guard_sleepiest_minutes.iter().max_by_key(|(_g, &(_m, n))| n).unwrap();

    sleepiest_guard_minute.0.0 * sleepiest_guard_minute.1.0 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
[1518-11-01 00:00] Guard #10 begins shift
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

    #[test]
    fn test_parsing() {
        let log = parse(SAMPLE);

        assert!(log ==
            [
                LogLine { guard: Guard(10), state: SleepWake::Sleep, minute: 5 },
                LogLine { guard: Guard(10), state: SleepWake::Wake, minute: 25 },
                LogLine { guard: Guard(10), state: SleepWake::Sleep, minute: 30 },
                LogLine { guard: Guard(10), state: SleepWake::Wake, minute: 55 },
                LogLine { guard: Guard(99), state: SleepWake::Sleep, minute: 40 },
                LogLine { guard: Guard(99), state: SleepWake::Wake, minute: 50 },
                LogLine { guard: Guard(10), state: SleepWake::Sleep, minute: 24 },
                LogLine { guard: Guard(10), state: SleepWake::Wake, minute: 29 },
                LogLine { guard: Guard(99), state: SleepWake::Sleep, minute: 36 },
                LogLine { guard: Guard(99), state: SleepWake::Wake, minute: 46 },
                LogLine { guard: Guard(99), state: SleepWake::Sleep, minute: 45 },
                LogLine { guard: Guard(99), state: SleepWake::Wake, minute: 55 },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let log = parse(SAMPLE);
        assert_eq!(part1(&log), 240);
    }

    #[test]
    fn test_part2() {
        let log = parse(SAMPLE);
        assert_eq!(part2(&log), 4455);
    }
}