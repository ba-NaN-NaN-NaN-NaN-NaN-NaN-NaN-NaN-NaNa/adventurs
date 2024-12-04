#[allow(dead_code)]
pub struct Grid2d {
    pub tiles: Vec<Vec<u8>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GridDirs8 {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Grid2d {
    pub fn from_lines(content: &str) -> Grid2d {
        let lines: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles: Vec<Vec<u8>>   = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<u8> = Vec::new();
            for ch in trimmed.chars() {
                tile_row.push(ch as u8);
            }
            tiles.push(tile_row);
        }

        let num_rows = tiles.len();
        let num_cols = tiles[0].len();

        for tile_row in tiles.iter() {
            if tile_row.len() != num_cols {
                panic!("lkgfjdh")
            }
        }

        Grid2d{
                tiles,
                num_rows,
                num_cols
        }
    }
}