use std::{collections::HashSet, fmt::Display, mem::swap};

struct Die {
    id: usize,
    faces: Vec<i64>,
    seed: i64,

    pulse: i64,
    roll_number: i64,
}

impl Die {
    fn parse(s: &str) -> Self {
        // 1: faces=[1,2,4,-1,5,7,9] seed=3
        let (num, data) = s.trim().split_once(": ").unwrap();
        let id = num.parse().unwrap();

        let (faces, seed) = data.split_once(" ").unwrap();
        let mut faces: Vec<i64> = faces
            .strip_prefix("faces=[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let seed = seed.strip_prefix("seed=").unwrap().parse().unwrap();
        faces.shrink_to_fit();

        Self {
            id,
            faces,
            seed,
            pulse: seed,
            roll_number: 0,
        }
    }

    fn roll(&mut self) -> i64 {
        self.roll_number += 1;
        let spin = self.roll_number * self.pulse;
        let l = self.faces.len();
        self.faces.rotate_left(usize::try_from(spin).unwrap() % l);
        let result = self.faces[0];
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_number + self.seed;
        result
    }
}

fn split_emptyline(s: &str) -> (&str, &str) {
    s.split_once("\n\n").or_else(|| s.split_once("\r\n\r\n")).unwrap()
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    let part1 = solve_part1();
    let part2 = solve_part2();
    let part3 = solve_part3();

    (part1, part2, part3)
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut dice: Vec<Die> = include_str!("part1.txt").lines().map(Die::parse).collect();
    let mut total = 0;

    (1usize..)
        .find(|_| {
            total += dice.iter_mut().map(|die| die.roll()).sum::<i64>();
            total > 10_000
        })
        .unwrap()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (dice, track) = split_emptyline(include_str!("part2.txt"));
    let track: Vec<i64> = track.trim().bytes().map(|n| (n - b'0') as i64).collect();

    let mut dice: Vec<Die> = dice.lines().map(Die::parse).collect();
    let mut positions = vec![0usize; dice.len()];
    let mut result = Vec::new();

    while !positions.iter().all(|&i| i == track.len()) {
        dice.iter_mut().zip(positions.iter_mut()).for_each(|(d, p)| {
            if *p != track.len() && d.roll() == track[*p] {
                *p += 1;
                if *p == track.len() {
                    result.push(d.id);
                }
            }
        })
    }

    result
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (dice, track) = split_emptyline(include_str!("part3.txt"));

    let width = track.lines().next().unwrap().len();
    let map: Vec<i64> = track
        .trim()
        .bytes()
        .filter(|n| !matches!(n, b'\n' | b'\r'))
        .map(|n| (n - b'0') as i64)
        .collect();
    let mut visited = grid::Grid::from_vec(vec![false; map.len()], width);
    let map = grid::Grid::from_vec(map, width);

    let dice: Vec<Die> = dice.lines().map(Die::parse).collect();
    for mut die in dice {
        let first_roll = die.roll();

        let mut states: HashSet<(usize, usize)> = map
            .indexed_iter()
            .filter_map(|(pos, &cell)| (cell == first_roll).then_some(pos))
            .collect();
        let mut next_states = HashSet::new();

        while !states.is_empty() {
            states.iter().for_each(|&pos| visited[pos] = true);

            let next_roll = die.roll();

            next_states.extend(
                states
                    .drain()
                    .flat_map(|pos| {
                        [
                            Some(pos),
                            Some((pos.0 + 1, pos.1)),
                            pos.0.checked_sub(1).map(|fst| (fst, pos.1)),
                            Some((pos.0, pos.1 + 1)),
                            pos.1.checked_sub(1).map(|snd| (pos.0, snd)),
                        ]
                    })
                    .flatten()
                    .filter(|pos| map.get(pos.0, pos.1) == Some(&next_roll)),
            );

            swap(&mut states, &mut next_states);
        }
    }

    visited.into_iter().filter(|&b| b).count()
}
