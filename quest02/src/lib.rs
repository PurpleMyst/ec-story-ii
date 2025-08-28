use std::{collections::VecDeque, fmt::Display};

// Pretty similiar to AoC 2016 Day 19, so I reused the same approach that seems/seemed to work well
// there. The idea is to use two VecDeques to represent the circle, and keep them balanced. Well,
// the right half is actually a Vec, but we only ever push/pop from the end, so it's better.

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

#[inline]
pub fn solve_part1() -> u32 {
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
pub fn solve_part2() -> u32 {
    solve_part23(include_str!("part2.txt"), 100)
}

#[inline]
pub fn solve_part3() -> u32 {
    solve_part23(include_str!("part3.txt"), 100_000)
}

fn solve_part23(input: &str, repeats: usize) -> u32 {
    let mut left = input
        .trim()
        .bytes()
        .cycle()
        .take((input.trim().len() * repeats) / 2 + 1)
        .collect::<VecDeque<_>>();
    let mut right = input
        .trim()
        .bytes()
        .cycle()
        .skip(left.len())
        .take((input.trim().len() * repeats) - left.len())
        .collect::<Vec<_>>();
    right.reverse();

    let mut bolts = 0;

    // This seems to be significantly faster than b"RGB"[bolts % 3] ¯\_(ツ)_/¯
    let mut bolt_iter = b"RGB".into_iter().cycle().copied();

    while let Some(head) = left.pop_front() {
        if head == bolt_iter.next().unwrap() {
            if left.len() < right.len() {
                right.pop().unwrap();
            }
        }

        if left.len() < right.len() {
            left.push_back(right.pop().unwrap());
        }

        bolts += 1;
    }

    bolts
}
