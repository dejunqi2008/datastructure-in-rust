pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut res = 0;
        let mut visited = vec![vec![false; col]; row];
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' && !visited[i][j] {
                    Self::helper(&mut visited, i, j, &grid);
                    res += 1;
                }
            }
        }
        return res;
    }

    fn helper(visited: &mut Vec<Vec<bool>>, i: usize, j: usize, grid: &Vec<Vec<char>>) {
        visited[i][j] = true;
        let row = grid.len();
        let col = grid[0].len();
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let kx = i as i32;
        let ky = j as i32;
        
        for [u, v] in dirs {
            let x = (kx + u) as usize;
            let y = (ky + v) as usize;
            if x < row && y < col && !visited[x][y] && grid[x][y] == '1' {
                Self::helper(visited, x as usize, y as usize, grid);
            }
        }
    }
}
