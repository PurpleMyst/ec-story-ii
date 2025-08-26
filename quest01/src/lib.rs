use std::fmt::Display;

use itertools::Itertools;

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
    fn simulate(&self, slot_idx: usize, moves: &str) -> usize {
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
        (2 * final_slot).saturating_sub(initial_slot)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

pub fn solve_part1() -> usize {
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

pub fn solve_part2() -> usize {
    let (board, tokens) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let board = Board::new(board);

    tokens
        .lines()
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

    let slot_values = tokens
        .lines()
        .map(|moves| {
            (0..=board.width / 2)
                .map(|slot_idx| board.simulate(slot_idx, moves))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (min, max) = (0..=board.width / 2)
        .permutations(6)
        .map(|perm| {
            perm.into_iter()
                .enumerate()
                .map(|(token_idx, slot_idx)| slot_values[token_idx][slot_idx])
                .sum::<usize>()
        })
        .minmax()
        .into_option()
        .unwrap();

    format!("{min} {max}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3() {
        assert_eq!(solve_part3(include_str!("sample2_part3.txt")).to_string(), "25 66");
        assert_eq!(solve_part3(include_str!("sample3_part3.txt")).to_string(), "39 122");
    }
}
