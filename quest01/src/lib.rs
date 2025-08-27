use std::fmt::Display;

use num_traits::PrimInt;
use rayon::prelude::*;

const P3_NUM_TOKENS: usize = 6;
const P3_NUM_SLOTS: usize = 20;

struct Board<Row> {
    width: usize,
    rows: Vec<Row>,
}

enum Direction {
    Left,
    Right,
}

fn split_emptyline(s: &str) -> (&str, &str) {
    s.split_once("\n\n").or_else(|| s.split_once("\r\n\r\n")).unwrap()
}

impl<Row: PrimInt> Board<Row> {
    fn new(data: &'static str) -> Self {
        let mut it = data.lines().peekable();

        let width = it.peek().unwrap().len();
        debug_assert!(
            width <= (Row::max_value().count_ones() as usize),
            "{width} > {}",
            Row::max_value().count_ones()
        );

        let rows = it
            .map(|line| {
                line.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'*').then_some(Row::one() << i))
                    .fold(Row::zero(), |acc, cell| acc | cell)
            })
            .collect();

        Self { width, rows }
    }

    fn has_nail(&self, x: usize, y: usize) -> bool {
        !(self.rows[y] & (Row::one() << x)).is_zero()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    /// Simulate a token falling through the board, starting at the given slot index (0-based).
    /// Returns the number of coins won.
    fn simulate(&self, slot_idx: usize, moves: &str) -> u16 {
        let mut x = 2 * slot_idx;
        debug_assert!(x < self.width);

        let mut it = parse_moves(moves);

        for y in 0..self.height() {
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
    let (board, tokens) = split_emptyline(include_str!("part1.txt"));
    let board = Board::<u32>::new(board);

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
    let (board, tokens) = split_emptyline(include_str!("part2.txt"));
    let board = Board::<u32>::new(board);

    tokens
        .par_lines()
        .map(|moves| {
            (0..=board.width / 2)
                .into_par_iter()
                .map(|slot_idx| board.simulate(slot_idx, moves))
                .max()
                .unwrap()
        })
        .sum()
}

/// Hungarian algorithm for rectangular matrices (min-cost assignment).
/// `cost` is an n x m matrix (n = #rows, m = #cols), with n <= m.
/// Returns min_cost
/// This is an O(n*m^2) implementation (or O(n^3) for square matrices) based on
/// the "assignment by potentials" method, optimized for performance.
fn hungarian_rect(cost: &[[i32; P3_NUM_SLOTS]]) -> i32 {
    // Potentials for rows (u) and columns (v).
    let mut u = [0i32; P3_NUM_TOKENS];
    let mut v = [0i32; P3_NUM_SLOTS];

    // p[j] = row assigned to column j. usize::MAX if unassigned.
    let mut p = [usize::MAX; P3_NUM_SLOTS];

    // Pre-allocate vectors to avoid reallocation inside the main loop.
    let mut minv = [0i32; P3_NUM_SLOTS];
    let mut way = [usize::MAX; P3_NUM_SLOTS];

    // For each row, find an augmenting path to an unassigned column.
    for i in 0..P3_NUM_TOKENS {
        // Reset per-row data structures.
        let mut used = 0;
        minv.fill(i32::MAX);

        // Sentinel column for rooting the augmenting path search.
        let mut j0 = usize::MAX;
        let mut current_row = i;

        // Core loop: find the cheapest augmenting path from `current_row`.
        // This is a variation of Dijkstra's algorithm on the residual graph.
        loop {
            // Mark the current column's path as explored.
            if j0 != usize::MAX {
                used |= 1 << j0;
            }

            // Expand from the current row to find the minimum slack.
            let mut delta = i32::MAX;
            let mut j1 = usize::MAX;

            for j in 0..P3_NUM_SLOTS {
                if (used & (1 << j)) == 0 {
                    // Calculate slack: cost[row][j] - u[row] - v[j]
                    let cur = cost[current_row][j] - u[current_row] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }

            // Update potentials to introduce new zero-slack edges.
            for j in 0..P3_NUM_SLOTS {
                if (used & (1 << j)) != 0 {
                    if let Some(row) = p.get(j).filter(|&&r| r != usize::MAX) {
                        u[*row] += delta;
                    }
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            // If the column with minimum slack is unassigned, we found an augmenting path.
            if p[j0] == usize::MAX {
                break;
            }
            // Otherwise, extend the alternating tree.
            current_row = p[j0];
        }

        // Augment the path by flipping edges from j0 back to the root.
        loop {
            if j0 == usize::MAX {
                break;
            }
            let j_prev = way[j0];
            let row_to_match = if j_prev == usize::MAX {
                i // The root of the path
            } else {
                p[j_prev]
            };
            p[j0] = row_to_match;
            j0 = j_prev;
        }
    }

    // Calculate the total cost of the optimal assignment.
    let mut result = 0;

    for j in 0..P3_NUM_SLOTS {
        if let Some(row) = p.get(j).filter(|&&r| r != usize::MAX) {
            result += cost[*row][j];
        }
    }

    result
}

pub fn solve_part3() -> impl Display {
    let (board, tokens) = split_emptyline(include_str!("part3.txt"));
    let board = Board::<u64>::new(board);

    let token_slot_values: Vec<[i32; P3_NUM_SLOTS]> = tokens
        .lines()
        .map(|moves| std::array::from_fn(|slot_idx| board.simulate(slot_idx, moves) as i32))
        .collect();

    // 1) Minimize total coins directly.
    let min = hungarian_rect(&token_slot_values);
    // 2) Maximize total coins by minimizing the transformed cost.
    // cost' = C - value, where C is a large constant (e.g., max_value).
    let max = solve_part3_max(token_slot_values);

    format!("{} {}", min, max)
}

fn solve_part3_max(mut token_slot_values: Vec<[i32; P3_NUM_SLOTS]>) -> i32 {
    let max_val = token_slot_values
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .copied()
        .unwrap_or(0);

    token_slot_values
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|v| *v = max_val - *v));

    let min_transformed_total = hungarian_rect(&token_slot_values);
    (P3_NUM_TOKENS as i32) * max_val - min_transformed_total
}
