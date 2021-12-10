use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug, Clone)]
struct BingoBoard {
    nums: Vec<Vec<u32>>,
    marks: Vec<Vec<bool>>,
}

const BOARD_SIZE: usize = 5;

impl BingoBoard {
    pub fn new(nums: Vec<Vec<u32>>) -> Self {
        Self {
            nums,
            marks: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn at(&self, x: usize, y: usize) -> (u32, bool) {
        (self.nums[y][x], self.marks[y][x])
    }

    pub fn mark(&mut self, num: u32) {
        for (y, row) in self.nums.iter().enumerate() {
            for (x, n) in row.iter().enumerate() {
                if *n == num {
                    self.marks[y][x] = true;
                }
            }
        }
    }

    pub fn wins(&self) -> bool {
        // Check Rows
        let rows_win = self
            .marks
            .iter()
            .map(|row| row.iter().map(|c| if *c { 1 } else { 0 }).sum::<usize>() == BOARD_SIZE)
            .any(|r| r);

        let columns_win = (0..BOARD_SIZE)
            .map(|x| {
                (0..BOARD_SIZE)
                    .map(|y| if self.marks[y][x] { 1 } else { 0 })
                    .sum::<usize>()
                    == BOARD_SIZE
            })
            .any(|c| c);

        rows_win || columns_win
    }

    pub fn unmarked_sum(&self) -> u32 {
        (0..BOARD_SIZE)
            .map(|y| {
                (0..BOARD_SIZE)
                    .map(|x| self.at(x, y))
                    .filter(|(_, mark)| !mark)
                    .map(|(n, _)| n)
                    .sum::<u32>()
            })
            .sum()
    }
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut lines = reader.lines();
    let called_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut cur_board = Vec::new();
    for line_res in lines {
        let line = line_res.unwrap();
        if line.is_empty() {
            boards.push(BingoBoard::new(cur_board));
            cur_board = Vec::new();
        } else {
            cur_board.push(
                line.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            )
        }
    }

    // Part 1/2
    let mut win_order = Vec::new();
    let mut won_on = vec![0; boards.len()];
    for num in called_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if win_order.contains(&i) {
                continue;
            }
            board.mark(num);
            if board.wins() {
                win_order.push(i);
                won_on[i] = num;
            }
        }
    }

    let winner_idx = win_order[0];
    let winner = &boards[winner_idx];
    let winner_won_on = won_on[winner_idx];
    println!("{}", winner.unmarked_sum() * winner_won_on);

    let loser_idx = win_order[win_order.len() - 1];
    let loser = &boards[loser_idx];
    let loser_won_on = won_on[loser_idx];
    println!("{}", loser.unmarked_sum() * loser_won_on);
}
