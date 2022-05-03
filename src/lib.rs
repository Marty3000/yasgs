//! yasgs (yet another sodoku game solver)
//!
//! this lib will solve sudokus, but it will not only work on "standard" 9x9 sudokus,\
//! but also on a 4x4, 16x16, 25x25, 36x36 and 49x49 sudoku.
#![forbid(unsafe_code)]

mod sudoku;

const ABC: &str = ".123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";

/// check the input
///
/// returns to get the size and teh initial state of the sudoku
fn init(inp: &str) -> (usize, String) {
    let mut sudo: String;
    for i in [7, 6, 5, 4, 3, 2] {
        //println!("  abc: {:?}", abc);
        sudo = String::new();
        for mut c in inp.chars() {
            if c == '0' {
                c = '.'
            }
            if ABC[..((i * i) + 1)].chars().filter(|x| *x == c).count() == 1 {
                sudo.push(c);
            }
        }
        if sudo.len() == i * i * i * i {
            return (i, sudo);
        }
    }
    (0, String::from(""))
}

/// format the solutions
///
/// based on the initial input
fn format_result(size: usize, inp: &str, sol_vec: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for sol in sol_vec {
        let mut sol_form: String = String::from("");
        let mut sol_rev = sol.chars().rev().collect::<String>();
        for mut c in inp.chars() {
            //for c in inp.chars() {
            if c == '0' {
                c = '.'
            }
            if ABC[..((size * size) + 1)]
                .chars()
                .filter(|x| *x == c)
                .count()
                == 1
            {
                sol_form.push(sol_rev.pop().unwrap());
            } else {
                sol_form.push(c);
            }
        }
        res.push(sol_form);
    }
    res
}

/// solve a sudoku
///
/// input:\
/// - description of the sudoku
/// - number of solutions to return
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
            solve("1234341.........", 2),
            vec![
                String::from("1234341221434321"),
                String::from("1234341223414123")
            ]
        );
        assert_eq!(
            solve("1234341000000000", 2),
            vec![
                String::from("1234341221434321"),
                String::from("1234341223414123")
            ]
        );
    }

    #[test]
    fn test_9x9_format() {
        assert_eq!(
            solve(
                "
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
  ╚═══════════╩═══════════╩═══════════╝",
                3
            ),
            vec![String::from(
                "
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
  ╚═══════════╩═══════════╩═══════════╝"
            )]
        );
    }

    #[test]
    fn test_error() {
        // 5 not allowed as stone for a 4x4-board => input only 15 chars length
        assert_eq!(
            solve("1234341....5....", 1),
            vec![String::from("Invalid input.")]
        );

        // input 17 chars length => does not fit into a 4x4-board
        assert_eq!(
            solve("1234341..........", 1),
            vec![String::from("Invalid input.")]
        );

        // input 15 chars length => does not fit into a 4x4-board
        assert_eq!(
            solve("1234341........", 1),
            vec![String::from("Invalid input.")]
        );
    }
}
