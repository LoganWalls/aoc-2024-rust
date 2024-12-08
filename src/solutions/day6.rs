use std::collections::HashMap;

use bitvec::bitvec;
use bitvec::vec::BitVec;

use super::Solution;

pub struct Day6;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn step_forward(&self, (x, y): (usize, usize)) -> Coord {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }

    fn step_back(&self, (x, y): (usize, usize)) -> Coord {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x - 1, y),
            Direction::West => (x + 1, y),
        }
    }
}

impl From<Direction> for usize {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Debug, Clone)]
struct BitCube {
    pub width: usize,
    pub depth: usize,
    pub bits: BitVec,
}

impl BitCube {
    fn new(height: usize, width: usize, depth: usize) -> Self {
        Self {
            width,
            depth,
            bits: bitvec![0; height * width * depth],
        }
    }

    fn i(&self, x: usize, y: usize, z: usize) -> usize {
        x + self.width * y + self.width * self.depth * z
    }

    fn set(&mut self, x: usize, y: usize, z: usize, value: bool) {
        let i = self.i(x, y, z);
        self.bits.set(i, value);
    }

    fn set_slice(&mut self, start: (usize, usize), end: (usize, usize), z: usize, value: bool) {
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                self.set(x, y, z, value);
            }
        }
    }

    fn get(&mut self, x: usize, y: usize, z: usize) -> bool {
        let i = self.i(x, y, z);
        *self.bits.get(i).expect("valid indices")
    }
}

fn find_next_barrier(
    (x, y): (usize, usize),
    direction: &Direction,
    barriers: &[usize],
) -> Option<(usize, usize)> {
    match direction {
        Direction::North => barriers
            .iter()
            .rev()
            .find(|b| b <= &&y)
            .map(|new_y| (x, *new_y)),
        Direction::South => barriers.iter().find(|b| b >= &&y).map(|new_y| (x, *new_y)),
        Direction::East => barriers.iter().find(|b| b >= &&x).map(|new_x| (*new_x, y)),
        Direction::West => barriers
            .iter()
            .rev()
            .find(|b| b <= &&x)
            .map(|new_x| (*new_x, y)),
    }
}

fn next_path(
    position: Coord,
    direction: Direction,
    state: &State,
) -> (Option<Coord>, Coord, (Coord, Coord)) {
    let barrier = match direction {
        Direction::North | Direction::South => state
            .barriers_x
            .get(&position.0)
            .and_then(|barriers| find_next_barrier(position, &direction, barriers)),
        Direction::East | Direction::West => state
            .barriers_y
            .get(&position.1)
            .and_then(|barriers| find_next_barrier(position, &direction, barriers)),
    };

    let destination = match barrier {
        Some(b) => direction.step_back(b),
        None => match direction {
            Direction::North => (position.0, 0),
            Direction::South => (position.0, state.height - 1),
            Direction::East => (state.width - 1, position.1),
            Direction::West => (0, position.1),
        },
    };
    let path = match direction {
        Direction::North | Direction::West => (destination, position),
        Direction::South | Direction::East => (position, destination),
    };
    (barrier, destination, path)
}

type Coord = (usize, usize);

struct State {
    pub height: usize,
    pub width: usize,
    pub position: Coord,
    pub direction: Direction,
    pub barriers_x: HashMap<usize, Vec<usize>>,
    pub barriers_y: HashMap<usize, Vec<usize>>,
}

fn parse_input(input: &str) -> State {
    let mut height = 0;
    let mut width = 0;
    let mut position = None;
    let mut direction = None;
    let mut barriers_x = HashMap::<usize, Vec<usize>>::new();
    let mut barriers_y = HashMap::<usize, Vec<usize>>::new();
    for (y, line) in input.lines().enumerate() {
        width = line.len();
        height += 1;

        for (x, b) in line.bytes().enumerate() {
            match &[b; 1] {
                b"#" => {
                    barriers_x.entry(x).or_default().push(y);
                    barriers_y.entry(y).or_default().push(x);
                }
                b"." => (),
                d => {
                    position = Some((x, y));
                    direction = Some(match d {
                        b"^" => Direction::North,
                        b"v" => Direction::South,
                        b">" => Direction::East,
                        b"<" => Direction::West,
                        _ => panic!("Unexpected character: {}", b),
                    });
                }
            }
        }
    }
    State {
        height,
        width,
        position: position.expect("found start"),
        direction: direction.expect("found start"),
        barriers_x,
        barriers_y,
    }
}

impl Solution for Day6 {
    fn input() -> &'static str {
        include_str!("../../inputs/day6.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day6.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        let mut state = parse_input(input);
        let mut grid = BitCube::new(state.height, state.width, 1);
        loop {
            let (barrier, destination, (start, end)) =
                next_path(state.position, state.direction, &state);

            grid.set_slice(start, end, 0, true);

            if barrier.is_none() {
                break Ok(grid.bits.count_ones());
            }

            state.position = destination;
            state.direction = state.direction.turn();
        }
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        let mut state = parse_input(input);
        let mut loop_barriers = BitCube::new(state.height, state.width, 1);
        let mut loop_states = BitCube::new(state.height, state.width, 4);
        let mut history = BitCube::new(state.height, state.width, 4);

        loop {
            let (barrier, destination, (start, end)) =
                next_path(state.position, state.direction, &state);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    history.set(x, y, state.direction.into(), true);

                    let (bx, by) = state.direction.step_forward((x, y));
                    if (bx, by) == state.direction.step_forward(destination) {
                        continue;
                    }

                    // Simulate if we were to add a barrier at (bx, by)...
                    let mut sim_history = history.clone();
                    let mut sim_position = (x, y);
                    let mut sim_direction = state.direction.turn();
                    loop {
                        let (sim_barrier, (sim_x, sim_y), (sim_start, sim_end)) =
                            next_path(sim_position, sim_direction, &state);

                        if sim_barrier.is_none() {
                            break;
                        }
                        let visited_before = sim_history.get(sim_x, sim_y, sim_direction.into());

                        if visited_before {
                            loop_barriers.set(bx, by, 0, true);
                            dbg!(visited_before);
                            dbg!((bx, by));
                            dbg!((sim_x, sim_y, sim_direction));
                            loop_states.set(x, y, state.direction.turn().into(), true);
                            break;
                        }

                        sim_history.set_slice(sim_start, sim_end, sim_direction.into(), true);
                        sim_position = (sim_x, sim_y);
                        sim_direction = sim_direction.turn();
                    }
                }
            }

            if barrier.is_none() {
                break Ok(loop_barriers.bits.count_ones());
            }

            state.position = destination;
            state.direction = state.direction.turn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day6::example_input();
        let result = Day6::part1(input).unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn part2() {
        let input = Day6::example_input();
        let result = Day6::part2(input).unwrap();
        assert_eq!(result, 6);
    }
}

// 4342 too high
// 3835 wrong, probably too high?
//
// Barriers:
// (3, 6)
// (6, 7)
// (7, 7)
// (1, 8)
// (3, 8)
// (7, 9)
//
//
// (6, 6) is the extra one / wrong one.
