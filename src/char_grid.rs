use std::collections::HashMap;

pub struct CharGrid {
    grid_map: HashMap<char, Vec<Vec<i32>>>, 
}

impl CharGrid {
    pub fn new() -> CharGrid {
        let mut g = HashMap::new();
        g.insert('a', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('b', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('c', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('d', vec![vec![0, 0, 0, 0],
                           vec![0, 0, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('e', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('f', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('g', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('h', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('i', vec![vec![0, 0, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('j', vec![vec![0, 0, 0, 0],
                           vec![0, 0, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('k', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('l', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('m', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('n', vec![vec![0, 0, 0, 0],
                           vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('o', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('p', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('q', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 1]]);

        g.insert('r', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 1, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('s', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('t', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('u', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('v', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 1, 0, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('w', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('x', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 1, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('y', vec![vec![0, 0, 0, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);

        g.insert('z', vec![vec![0, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 1, 0],
                           vec![0, 1, 0, 0],
                           vec![1, 0, 0, 0],
                           vec![1, 1, 1, 0],
                           vec![0, 0, 0, 0]]);
        
        CharGrid {
            grid_map: g
        }
    }
    pub fn get_grid_for(&self, c: char) -> &Vec<Vec<i32>> {
        &self.grid_map[&c]
    }
}
