use aoc2021::commons::io::load_argv_lines;
use cached::proc_macro::cached;
use lazy_static::lazy_static;

lazy_static! {
    static ref DICE_OUTCOMES: Vec<(u16, u8)> = {
        let mut outcomes = [0; 10];
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    outcomes[i + j + k] += 1;
                }
            }
        }
        outcomes
            .iter()
            .enumerate()
            .filter(|(_, s)| **s > 0)
            .map(|(s, c)| (s as u16, *c))
            .collect()
    };
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Player {
    pos: u16,
    score: u16,
}

fn part1(mut p1: Player, mut p2: Player) -> u32 {
    let mut die = 0;
    let mut rolls = 0;
    let (mut active_player, next_player) = (&mut p1, &mut p2);
    loop {
        let mut roll: u16 = 0;
        for _ in 0..3 {
            die += 1;
            roll += die;
            die %= 100;
        }
        rolls += 3;
        active_player.pos = (active_player.pos + roll) % 10;
        active_player.score += active_player.pos as u16 + 1;
        if active_player.score >= 1000 {
            return next_player.score as u32 * rolls as u32;
        }
        std::mem::swap(active_player, next_player);
    }
}

#[cached]
fn part2(p1: Player, p2: Player, first_player: bool) -> (u64, u64) {
    if p1.score >= 21 {
        return (1, 0);
    }
    if p2.score >= 21 {
        return (0, 1);
    }
    let (player, next_player) = if first_player { (p1, p2) } else { (p2, p1) };
    let mut subwins = (0, 0);
    for (roll, count) in DICE_OUTCOMES.iter() {
        let mut active_player = player.clone();
        active_player.pos = (active_player.pos + roll) % 10;
        active_player.score += active_player.pos as u16 + 1;
        let roll_wins = if first_player {
            part2(active_player, next_player.clone(), !first_player)
        } else {
            part2(next_player.clone(), active_player, !first_player)
        };
        subwins.0 += roll_wins.0 * (*count as u64);
        subwins.1 += roll_wins.1 * (*count as u64);
    }
    subwins
}

fn main() {
    let mut players = load_argv_lines::<String>().map(|x| {
        let x = x.unwrap();
        let (_, num) = x.split_once(": ").unwrap();
        Player {
            pos: num.parse::<u16>().unwrap() - 1,
            score: 0,
        }
    });
    let p1 = players.next().unwrap();
    let p2 = players.next().unwrap();
    println!("{}", part1(p1.clone(), p2.clone()));
    let wins = part2(p1, p2, true);
    println!("{:?}", wins.0.max(wins.1));
}
