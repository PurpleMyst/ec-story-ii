use std::{fmt::Display, mem::swap};

use rayon::prelude::*;
use rustc_hash::FxHashMap;

struct Board {
    width: usize,
    rows: Vec<u64>,
}

enum Direction {
    Left,
    Right,
}

impl Board {
    fn new(data: &'static str) -> Self {
        let mut it = data.lines().peekable();

        let width = it.peek().unwrap().len();
        debug_assert!(width <= 64);

        let rows = it
            .map(|line| {
                line.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'*').then_some(1 << i))
                    .fold(0, |acc, cell| acc | cell)
            })
            .collect();

        Self { width, rows }
    }

    fn has_nail(&self, x: usize, y: usize) -> bool {
        self.rows[y] & (1 << x) != 0
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    /// Simulate a token falling through the board, starting at the given slot index (0-based).
    /// Returns the number of coins won.
    fn simulate(&self, slot_idx: usize, moves: &str) -> u16 {
        let mut x = 2 * slot_idx;
        let mut y = 0;
        debug_assert!(x < self.width);

        let mut it = parse_moves(moves);

        while y < self.height() {
            if self.has_nail(x, y) {
                match it.next().unwrap() {
                    Direction::Left => {
                        if x != 0 {
                            x -= 1;
                        } else {
                            x += 1;
                        }
                    }
                    Direction::Right => {
                        if x != self.width - 1 {
                            x += 1;
                        } else {
                            x -= 1;
                        }
                    }
                }
            }

            y += 1;
        }

        let initial_slot = ((2 * slot_idx) / 2) + 1;
        let final_slot = (x / 2) + 1;
        (2 * final_slot).saturating_sub(initial_slot) as _
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

pub fn solve_part1() -> u16 {
    let (board, tokens) = include_str!("part1.txt").split_once("\n\n").unwrap();
    let board = Board::new(board);

    tokens
        .lines()
        .enumerate()
        .map(|(slot_idx, moves)| board.simulate(slot_idx, moves))
        .sum()
}

fn parse_moves(moves: &str) -> impl Iterator<Item = Direction> {
    moves.bytes().map(|b| match b {
        b'L' => Direction::Left,
        b'R' => Direction::Right,
        _ => unreachable!(),
    })
}

pub fn solve_part2() -> u16 {
    let (board, tokens) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let board = Board::new(board);

    tokens
        .par_lines()
        .map(|moves| {
            (0..=board.width / 2)
                .map(|slot_idx| board.simulate(slot_idx, moves))
                .max()
                .unwrap()
        })
        .sum()
}

pub fn solve_part3() -> impl Display {
    let (board, tokens) = include_str!("part3.txt").split_once("\n\n").unwrap();
    let board = Board::new(board);
    let num_slots = board.width / 2 + 1;

    let token_slot_values = tokens
        .lines()
        .map(|moves| {
            (0..num_slots)
                .map(|slot_idx| board.simulate(slot_idx, moves) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // States: used_slots -> (min_score, max_score)
    let mut states = FxHashMap::default();
    let mut next_states = FxHashMap::default();
    states.insert(0u32, (0u8, 0u8));

    // For each token, try placing it in each available slot; in each iteration, `states` contains
    // the best we've been able to do with the previous tokens.
    for slot_values in &token_slot_values {
        for (used_slots, (current_min, current_max)) in states.drain() {
            for slot in 0..num_slots {
                if (used_slots & (1 << slot)) != 0 {
                    continue;
                }
                let new_used_slots = used_slots | (1 << slot);
                let coins_won = slot_values[slot];

                let new_min = current_min + coins_won;
                let new_max = current_max + coins_won;

                let entry = next_states.entry(new_used_slots).or_insert((u8::MAX, 0));
                entry.0 = entry.0.min(new_min);
                entry.1 = entry.1.max(new_max);
            }
        }
        swap(&mut states, &mut next_states);
    }

    let mut min = u8::MAX;
    let mut max = 0;
    for (min_score, max_score) in states.into_values() {
        min = min.min(min_score);
        max = max.max(max_score);
    }
    format!("{min} {max}")
}
