#![forbid(unsafe_code)]

mod sudoku;

const ABC: &'static str = ".123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";


fn init(inp: &str) -> (usize, String) {
    let mut sudo: String;
    for i in [7, 6, 5, 4, 3, 2] {
        let abc: Vec<char> = ABC[..((i * i) + 1)].chars().collect();
        println!("  abc: {:?}", abc);
        sudo = String::new();
        for mut c in inp.chars() {
            if c == '0' {c = '.'}
            if abc.contains(&c) {
                sudo.push(c);
            }
        }
        if sudo.len() == i * i * i * i {
            return (i, sudo);
        }
    }
    (0, String::from(""))
}

fn format_result(size: usize, inp: &str, sol_vec: Vec<String>) -> Vec<String> {
    let abc: Vec<char> = ABC[..((size * size) + 1)].chars().collect();
    let mut res: Vec<String> = Vec::new();
    for sol in sol_vec {
        let mut sol_form: String = String::from("");
        let mut sol_rev = sol.chars().rev().collect::<String>();
        for mut c in inp.chars() {
        //for c in inp.chars() {
            if c == '0' {c = '.'}
            if abc.contains(&c) {
                sol_form.push(sol_rev.pop().unwrap());
            }
            else {
                sol_form.push(c);
            }

        }
        res.push(sol_form);
    }
    res
}

pub fn solve(inp: &str, cnt: usize) -> Vec<String> {
    let (size, sudo) = init(inp);
    if size == 0 {
        return vec![String::from("Invalid input.")];
    }
    let mut board = sudoku::Board::new(size, &sudo);
    format_result(size, inp, board.solve(cnt))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4() {
        assert_eq!(
            vec![String::from("1234341221434321"),
            String::from("1234341223414123")], solve("1234341.........", 2));
        assert_eq!(
            vec![String::from("1234341221434321"),
            String::from("1234341223414123")], solve("1234341000000000", 2));

    }

    #[test]
    fn test_9x9_format() {
        assert_eq!(
            vec![String::from("
  ╔═══════════╦═══════════╦═══════════╗
  ║ 1 │ 4 │ 2 ║ 8 │ 7 │ 3 ║ 9 │ 6 │ 5 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 5 │ 8 │ 7 ║ 6 │ 4 │ 9 ║ 2 │ 3 │ 1 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 9 │ 6 │ 3 ║ 1 │ 2 │ 5 ║ 8 │ 4 │ 7 ║
  ╠═══════════╬═══════════╬═══════════╣
  ║ 4 │ 2 │ 9 ║ 7 │ 3 │ 6 ║ 5 │ 1 │ 8 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 6 │ 3 │ 5 ║ 2 │ 1 │ 8 ║ 4 │ 7 │ 9 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 8 │ 7 │ 1 ║ 9 │ 5 │ 4 ║ 3 │ 2 │ 6 ║
  ╠═══════════╬═══════════╬═══════════╣
  ║ 2 │ 5 │ 6 ║ 3 │ 9 │ 1 ║ 7 │ 8 │ 4 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 7 │ 9 │ 8 ║ 4 │ 6 │ 2 ║ 1 │ 5 │ 3 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 3 │ 1 │ 4 ║ 5 │ 8 │ 7 ║ 6 │ 9 │ 2 ║
  ╚═══════════╩═══════════╩═══════════╝")], solve("
  ╔═══════════╦═══════════╦═══════════╗
  ║ 1 │ 4 │ . ║ . │ 7 │ . ║ . │ 6 │ 5 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 5 │ . │ . ║ . │ 4 │ . ║ . │ . │ 1 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 9 │ . │ . ║ . │ . │ . ║ . │ . │ 7 ║
  ╠═══════════╬═══════════╬═══════════╣
  ║ 4 │ 2 │ 9 ║ . │ . │ . ║ 5 │ 1 │ 8 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ . │ . │ 5 ║ 2 │ . │ 8 ║ 4 │ . │ . ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 8 │ 7 │ 1 ║ . │ . │ . ║ 3 │ 2 │ 6 ║
  ╠═══════════╬═══════════╬═══════════╣
  ║ 2 │ . │ . ║ . │ . │ . ║ . │ . │ 4 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 7 │ . │ . ║ . │ 6 │ . ║ . │ . │ 3 ║
  ║───┼───┼───║───┼───┼───║───┼───┼───║
  ║ 3 │ 1 │ . ║ . │ 8 │ . ║ . │ 9 │ 2 ║
  ╚═══════════╩═══════════╩═══════════╝", 3));
    }


    #[test]
    fn test_error() {
        assert_eq!(vec![String::from("Invalid input.")], solve("1234341....5....", 1));
        assert_eq!(vec![String::from("Invalid input.")], solve("1234341..........", 1));
        assert_eq!(vec![String::from("Invalid input.")], solve("1234341........", 1));
    }

}
