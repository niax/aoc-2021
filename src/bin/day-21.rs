use aoc2021::commons::io::load_argv_lines;

#[derive(Debug, Clone)]
struct Player {
    pos: u8,
    score: u16,
}

impl Player {
    pub fn has_won(&self) -> bool {
        self.score >= 1000
    }
}

fn part1(mut p1: Player, mut p2: Player) -> u32 {
    let mut die = 0;
    let mut rolls = 0;
    let mut players = [&mut p1, &mut p2];
    loop {
        for (i, player) in players.iter_mut().enumerate() {
            for _ in 0..3 {
                player.pos += die + 1;
                player.pos %= 10;
                rolls += 1;
                die += 1;
                die %= 100;
            }
            player.score += player.pos as u16 + 1;
            if player.has_won() {
                let loser = &players[if i == 0 { 1 } else { 0 }];
                return loser.score as u32 * rolls as u32;
            }
        }
    }
}

fn main() {
    let mut players = load_argv_lines::<String>().map(|x| {
        let x = x.unwrap();
        let (_, num) = x.split_once(": ").unwrap();
        Player {
            pos: num.parse::<u8>().unwrap() - 1,
            score: 0,
        }
    });
    let p1 = players.next().unwrap();
    let p2 = players.next().unwrap();
    println!("{}", part1(p1.clone(), p2.clone()));
}
