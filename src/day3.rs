use crate::err;
use regex::Regex;

struct Claim {
    id: usize,
    left_gap: usize,
    top_gap: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn horizontal_space(&self) -> usize {
        self.width + self.left_gap
    }

    fn vertical_space(&self) -> usize {
        self.height + self.top_gap
    }

    fn fill_board(&self, board_width: usize, board: &mut [usize]) {
        for i in 0..self.width {
            for j in 0..self.height {
                let x = self.left_gap + i;
                let y = self.top_gap + j;
                let idx = y * board_width + x;
                if board[idx] < 2 {
                    board[idx] += 1;
                }
            }
        }
    }
    fn no_overlap(&self, board_width: usize, board: &[usize]) -> bool {
        for i in 0..self.width {
            for j in 0..self.height {
                let x = self.left_gap + i;
                let y = self.top_gap + j;
                let idx = y * board_width + x;
                if board[idx] != 1 {
                    return false;
                }
            }
        }
        return true;
    }
}

fn parse_claim(line: &str, re: &Regex) -> Claim {
    let caps = re.captures(line).expect(err!());
    let id = caps
        .get(1)
        .expect(err!())
        .as_str()
        .parse::<usize>()
        .expect(err!());
    let left_gap = caps
        .get(2)
        .expect(err!())
        .as_str()
        .parse::<usize>()
        .expect(err!());
    let top_gap = caps
        .get(3)
        .expect(err!())
        .as_str()
        .parse::<usize>()
        .expect(err!());
    let width = caps
        .get(4)
        .expect(err!())
        .as_str()
        .parse::<usize>()
        .expect(err!());
    let height = caps
        .get(5)
        .expect(err!())
        .as_str()
        .parse::<usize>()
        .expect(err!());
    Claim {
        id,
        left_gap,
        top_gap,
        width,
        height,
    }
}

pub fn solve1(input: &str) -> usize {
    let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect(err!());
    let claims = input
        .lines()
        .map(|l| parse_claim(l, &regex))
        .collect::<Vec<_>>();
    let (height, width) = claims.iter().fold((0, 0), |(w, h), claim| {
        (
            w.max(claim.horizontal_space()),
            h.max(claim.vertical_space()),
        )
    });
    let mut board = vec![0; height * width];
    claims
        .iter()
        .for_each(|claim| claim.fill_board(width, &mut board));
    board.iter().filter(|&i| *i == 2).count()
}

pub fn solve2(input: &str) -> usize {
    let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect(err!());
    let claims = input
        .lines()
        .map(|l| parse_claim(l, &regex))
        .collect::<Vec<_>>();
    let (height, width) = claims.iter().fold((0, 0), |(w, h), claim| {
        (
            w.max(claim.horizontal_space()),
            h.max(claim.vertical_space()),
        )
    });
    let mut board = vec![0; height * width];
    claims
        .iter()
        .for_each(|claim| claim.fill_board(width, &mut board));
    claims
        .iter()
        .find(|claim| claim.no_overlap(width, &board))
        .map(|claim| claim.id)
        .expect(err!())
}

#[test]
fn test1() {
    let test_data = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(solve1(test_data), 4);
}
