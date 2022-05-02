#![forbid(unsafe_code)]

const ABC: &'static str = ".123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";

pub struct Board {
    size: usize,
    qsize: usize,
    field: Vec<Vec<u8>>,
    possible: Vec<Vec<Vec<u8>>>,
}

impl Board {

    pub fn new(qsize: usize, inp: &str) -> Board {
        let size = qsize * qsize;
        let abc: Vec<char> = ABC.chars().collect();
        let mut mabc = inp.chars().rev().collect::<String>();
        let mut me = Board {
            size: size,
            qsize: qsize,
            field: vec![vec![0; size]; size],
            possible: vec![vec![(1..(size as u8 + 1)).collect(); size]; size],
        };
        for y in 0..me.size {
            for x in 0..me.size {
                let inp_char = mabc.pop().unwrap();
                // println!("  Working on {} ...", inp_char);
                if abc.contains(&inp_char) {
                    // println!("    Found char {} => {}", inp_char, abc.iter().position(|x| *x == inp_char).unwrap());
                    me.field[y][x] = abc.iter().position(|x| *x == inp_char).unwrap() as u8
                }                
                if 0 < me.field[y][x] {
                    me.set_pfeld(x, y, me.field[y][x]);
                }
            }
        }
        me
    }

    pub fn solve(&mut self, num: usize) -> Vec<String> {
        let mut sol: Vec<String> = vec![];
        let mut obvi: bool = self.set_first_obvious();
        if !self.is_valid() {
            return sol;
        }
        while obvi {
            obvi = self.set_first_obvious();
            if !self.is_valid() {
                return sol;
            }
        }
        let (best_x, best_y) = self.list_best_guess();
        if best_x == self.size || best_y == self.size {
            if !self.is_valid() {
                return sol;
            }
            sol.push(self.print());
            return sol;
        }
        for best_val in &self.possible[best_y][best_x] {
            let mut nxt_move: Board = Board {
                size: self.size,
                qsize: self.qsize,
                field: self.field.clone(),
                possible: self.possible.clone(),
            };
            nxt_move.set_field(best_x, best_y, *best_val);
            if nxt_move.is_valid() {
                let mut nxt_sol = nxt_move.solve(num - sol.len());
                if !nxt_sol.is_empty() {
                    sol.append(&mut nxt_sol);
                    if num <= sol.len() {
                        return sol;
                    }
                }
            }
        }
        sol
    }

    // private functions start here

    fn set_field(&mut self, x: usize, y: usize, val: u8) {
        self.field[y][x] = val;
        self.set_pfeld(x, y, val);
    }

    fn set_pfeld(&mut self, x: usize, y: usize, val: u8) {
        self.possible[y][x] = vec![val];
        for i in 0..self.size {
            if i != x {
                // && self.possible[y][i].contains(&val) {
                self.possible[y][i].retain(|e| e != &val);
            }
            if i != y {
                //&&  self.possible[i][x].contains(&val) {
                self.possible[i][x].retain(|e| e != &val);
            }
        }
        let (x_caree, y_caree) = get_caree(self.qsize, x, y);
        for i in 0..(self.size - 1) {
            //            if self.possible[y_caree[i]][x_caree[i]].contains(&val) {
            self.possible[y_caree[i]][x_caree[i]].retain(|e| e != &val);
            //            }
        }
    }

    fn is_valid(&self) -> bool {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.possible[y][x].is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self) -> String {
        let abc: Vec<char> = ABC.chars().collect();
        let mut res: String = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                res.push(abc[self.field[y][x] as usize]);
            }
        }
        res
    }

    fn set_first_obvious(&mut self) -> bool {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.field[y][x] == 0 && self.possible[y][x].len() == 1 {
                    self.set_field(x, y, self.possible[y][x][0]);
                    return true;
                }
            }
        }
        false
    }

    fn list_best_guess(&self) -> (usize, usize) {
        let mut minp: usize = self.size + 1;
        let mut bg_x: usize = self.size;
        let mut bg_y: usize = self.size;
        for y in 0..self.size {
            for x in 0..self.size {
                let plen: usize = self.possible[y][x].len();
                if 1 < plen && plen < minp {
                    minp = plen;
                    bg_x = x;
                    bg_y = y;
                }
            }
        }
        (bg_x, bg_y)
    }
}

fn get_caree(qsize: usize, x: usize, y: usize) -> (Vec<usize>, Vec<usize>) {
    let mut x_caree: Vec<usize> = Vec::new();
    let mut y_caree: Vec<usize> = Vec::new();
    let x1: usize = x - (x % qsize);
    let y1: usize = y - (y % qsize);
    for i in 0..qsize {
        for j in 0..qsize {
            if x != x1 + i || y != y1 + j {
                x_caree.push(x1 + i);
                y_caree.push(y1 + j);
            }
        }
    }
    (x_caree, y_caree)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caree() {
        let (x_caree, y_caree) = get_caree(3, 1, 1);
        assert_eq!(vec![0, 0, 0, 1, 1, 2, 2, 2], x_caree);
        assert_eq!(vec![0, 1, 2, 0, 2, 0, 1, 2], y_caree);

        let (x_caree, y_caree) = get_caree(3, 3, 7);
        assert_eq!(vec![3, 3, 4, 4, 4, 5, 5, 5], x_caree);
        assert_eq!(vec![6, 8, 6, 7, 8, 6, 7, 8], y_caree);
    }

    #[test]
    fn test_4x4() {
        let mut test_board = Board::new(2, "1234341.........");
        let solu = test_board.solve(5); // only 4 solutions exist
        assert_eq!(
            vec![String::from("1234341221434321"),
            String::from("1234341223414123"),
            String::from("1234341241232341"),
            String::from("1234341243212143")],
            solu
        );
    }

    #[test]
    fn test_9x9() {
        let mut test_board = Board::new(3, ".....9.7.....82.5.327....4..16.4.....5....3......9.7.....6....58.2........42....8");
        let solu = test_board.solve(1);
        assert_eq!(
            vec![String::from(
                "685439271491782653327561849916347582758126394243895716139678425862954137574213968"
            )],
            solu
        );
    }

    #[test]
    fn multiple_solutions() {
        let cnt: usize = 10;
        let mut test_board = Board::new(3, "...............................................................8.2........42....8");
        let solu = test_board.solve(cnt);
        assert_eq!(cnt, solu.len());
    }
}
