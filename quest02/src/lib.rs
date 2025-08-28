use std::{collections::VecDeque, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

#[inline]
pub fn solve_part1() -> usize {
    let mut input = include_str!("part1.txt").trim().bytes().peekable();
    let mut bolts = 0;

    while input.peek().is_some() {
        for &bolt in b"RGB" {
            while input.peek().copied() == Some(bolt) {
                input.next();
            }
            input.next().unwrap();
            bolts += 1;
            if input.peek().is_none() {
                break;
            }
        }
    }

    bolts
}

#[inline]
pub fn solve_part2() -> usize {
    solve_part23(include_str!("part2.txt"), 100)
}

#[inline]
pub fn solve_part3() -> usize {
    solve_part23(include_str!("part3.txt"), 100_000)
}

fn solve_part23(input: &str, repeats: usize) -> usize {
    let mut circle = input.trim().as_bytes().repeat(repeats);
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    for item in circle.drain(0..circle.len() / 2) {
        left.push_back(item);
    }
    for item in circle.into_iter() {
        right.push_back(item);
    }

    let mut bolts = 0;

    while let Some(head) = left.pop_front() {
        if head == b"RGB"[bolts % 3] {
            if (left.len() + right.len() + 1) % 2 == 0 {
                right.pop_front().unwrap();
            }
        }

        if left.len() < right.len() {
            left.push_back(right.pop_front().unwrap());
        }

        bolts += 1;
    }

    bolts
}
