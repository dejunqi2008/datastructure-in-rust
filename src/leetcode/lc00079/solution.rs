pub struct Solution;

impl Solution {
    
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let row = board.len();
        let col = board[0].len();
        let chars: Vec<char> = word.chars().collect();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; col]; row];
        for i in 0..row {
            for j in 0..col {
                if board[i][j] == chars[0] {
                    if Self::helper(&mut visited, i as i32, j as i32, &board, &chars, 0) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn helper(
        visited: &mut Vec<Vec<bool>>, 
        i: i32,
        j: i32,
        board: &Vec<Vec<char>>,
        chars: &Vec<char>,
        idx: usize
    ) -> bool {
        let row = board.len();
        let col = board[0].len();
        
        if idx == chars.len() - 1 {
            return true;
        }
        visited[i as usize][j as usize] = true;
        print!("{}", chars[idx]);
        let dirs: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        for [u, v] in dirs {
            let x = (i + u) as usize;
            let y = (j + v) as usize;
            if x < row &&  y < col && !visited[x][y] && idx < chars.len() - 1 && board[x][y] == chars[idx + 1] {
                if Self::helper(visited, x as i32, y as i32, board, chars, idx + 1) {
                    return true;
                }
                
            }
        }
        visited[i as usize][j as usize] = false;
        println!("\n");
        return false;
    }


    pub fn test() {
        let mut board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let mut word = "ABCCED".to_string();
        let mut res = Self::exist(board, word);
        println!("\n{}", res);

        board = vec![vec!['a', 'a']];
        word = "aaa".to_string();
        res = Self::exist(board, word);
        println!("\n{}", res);
    }
}
