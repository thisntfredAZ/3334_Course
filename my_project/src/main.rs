use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}
use Direction::*;
#[derive(Debug, Clone, Copy)]
struct Action {
    direction: Direction,
    amount: i32,
}

fn parse(input: &str) -> Vec<Action> {
    let mut moves = vec![];
    for line in input.lines() {
        if line.is_empty() { continue; }
        let first = line.chars().next().unwrap();
        let direction = match first {
            'R' => Right,
            'L' => Left,
            _ => panic!(""),
        };
        let amount: i32 = line[1..].parse().unwrap();
        moves.push(Action{direction, amount});
    }
    return moves;
}
fn solve(moves: &Vec<Action>) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;

    for action in moves {
        match action.direction {
            Left => dial = (dial - action.amount).rem_euclid(100),
            Right => dial = (dial + action.amount).rem_euclid(100),
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}
fn main() {
    let input = fs::read_to_string("aocinput.txt").unwrap();
    let moves = parse(&input);
    let count = solve(&moves);
    println!("result = {}", count);
}