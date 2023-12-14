use core::num;
use std::collections::VecDeque;
use regex::Regex;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    appearance: char,
}

#[allow(dead_code)]
impl Tile {
    pub fn from_ch(ch: &char) -> Tile {
        Tile { appearance:*ch }
    }
}

#[allow(dead_code)]
pub struct Pattern {
    tiles: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl Pattern {
    pub fn from_content(content: &str) -> Pattern {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles:Vec<Vec<Tile>> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<Tile> = Vec::new();
            for ch in trimmed.chars() {
                tile_row.push(Tile::from_ch(&ch));
            }
            tiles.push(tile_row);
        }

        Pattern { 
            tiles: tiles,
        }
    }

    pub fn draw_glyphs(&self) -> String {
        let mut toreturn = "".to_string();

        for line in &self.tiles {
            let chars: String = line.iter().map( |t| t.appearance ).collect();
            toreturn = format!("{}\n{}", toreturn, chars);
        }
        toreturn.trim().to_string()
    }



    pub fn find_hline_reflect(&self, wanted_diffs:i64) -> Option<usize> {
        let candidates = self.find_hline_candidates();
        println!("find_hline_reflect() got hline_candidates = {:?}", candidates);
        for hline_split in &candidates {
            if wanted_diffs == self.validate_hline_reflect(*hline_split) {
                return Some(*hline_split)
            }
        }

        None
    }

    pub fn find_vline_reflect(&self, wanted_diffs:i64) -> Option<usize> {
        let candidates = self.find_vline_candidates();
        println!("find_vline_reflect() got vline_candidates = {:?}", candidates);
        for vline_split in &candidates {
            if wanted_diffs == self.validate_vline_reflect(*vline_split) {
                return Some(*vline_split)
            }
        }

        None
    }

    pub fn line_equality(&self, line_nr_a: usize, line_nr_b: usize) -> i64 {
        // Returns number of differing tiles
        let mut diffs = 0;
        for col_nr in 0..self.tiles.get(0).unwrap().len() {
            if self.tiles.get(line_nr_a).unwrap().get(col_nr).unwrap().appearance != 
               self.tiles.get(line_nr_b).unwrap().get(col_nr).unwrap().appearance {
                diffs += 1
            }
        }
        
        return diffs
    }


    pub fn column_equality(&self, col_nr_a: usize, col_nr_b: usize) -> i64 {
        // Returns number of differing tiles
        let mut diffs = 0;
        for line_nr in 0..self.tiles.len() {
            if self.tiles.get(line_nr).unwrap().get(col_nr_a).unwrap().appearance != 
               self.tiles.get(line_nr).unwrap().get(col_nr_b).unwrap().appearance {
                diffs += 1
            }
        }
        
        diffs
    }


    pub fn validate_hline_reflect(&self, line_nr: usize) -> i64 {
        // Horizontal reflection = between two rows.
        // Returns number of differing tiles
        let num_lines_above = line_nr+1; // Specifying line_nr = 0, means 1 line above.
        let num_lines_below = self.tiles.len()-num_lines_above;
        let num_rows_to_compare = num_lines_above.min(num_lines_below);

        let mut diffs:i64 = 0;

        if num_rows_to_compare == 0 {
            panic!("Can not compare 0 rows, got line_nr={}, splits into {}/{}, have {}", line_nr, num_lines_above, num_lines_below, self.tiles.len());
        }

        for n in 0..num_rows_to_compare {
            let line_above = line_nr-n;
            let line_below = line_nr+n+1;

            println!("Validating lines {}=={} for split at {} of {} lines.",line_above, line_below, line_nr, self.tiles.len());
            diffs += self.line_equality(line_above, line_below);
            if diffs > 5 {
                // Return early, we really only care about 0, 1, or many diffs
                return diffs
            }
        }

        diffs
    }


    pub fn validate_vline_reflect(&self, col_nr: usize) -> i64 {
        // Vertical reflection = between two column.
        // Returns number of differing tiles
        let mut diffs = 0;

        let num_cols_left = col_nr+1;
        let num_cols_right = self.tiles.get(0).unwrap().len()-num_cols_left;
        let num_cols_to_compare = num_cols_left.min(num_cols_right);
        if num_cols_to_compare == 0 {
            panic!("Can not compare 0 columns, got col_nr={}, splits into {}/{}, have {}", col_nr, num_cols_left, num_cols_right, self.tiles.get(0).unwrap().len());
        }

        for n in 0..num_cols_to_compare {
            let col_left = col_nr-n;
            let col_right = col_nr+n+1;

            // println!("Validating cols {}=={} for split at {} of {} columns.",col_left, col_right, col_nr, self.tiles.get(0).unwrap().len());
            diffs += self.column_equality(col_left, col_right);
            if diffs > 5 {
                // Return early, we really only care about 0, 1, or many diffs
                return diffs
            }
          
        }

        diffs
    }

    
    pub fn find_vline_candidates(&self) -> Vec<usize> {
        // Find candidate positions where left half mirrors right half.
        // the line of reflection is the vertical line between columns
        // 
        // vline <-> hmirror, specified by col_nr
        let mut toreturn: Vec<usize> = Vec::new();
        
        let line0: Vec<Tile> = self.tiles.get(0).unwrap().clone();
        let line1: Vec<Tile> = self.tiles.get(1).unwrap().clone();

        let chars0 = line0.iter().map( |t| t.appearance).collect();
        let chars1 = line1.iter().map( |t| t.appearance).collect();

        let cands0 = find_pairs(&chars0);
        let cands1 = find_pairs(&chars1);
        for cand in &cands0 {
            if cands1.contains(&cand) || true {
                toreturn.push(*cand)
            }
        }

        let str0:String = chars0.iter().collect();
        let str1:String = chars1.iter().collect();
        println!("find_vline_candidates() unioned {}->{:?} and {}->{:?} for {:?}", str0, cands0, str1, cands1, toreturn);

        toreturn
    }

    pub fn find_hline_candidates(&self) -> Vec<usize> {
        // Find candidate positions where top half mirrors bottom half.
        // the line of reflection is the horizontal line between rows.
        // 
        // vline <-> hmirror, specified by row_nr
        let mut toreturn: Vec<usize> = Vec::new();
        
        let mut col0: Vec<Tile> = Vec::new();
        let mut col1: Vec<Tile> = Vec::new();
        for line in &self.tiles {
            col0.push(line.get(0).unwrap().clone());
            col1.push(line.get(1).unwrap().clone());
        }

        let chars0 = col0.iter().map( |t| t.appearance).collect();
        let chars1 = col1.iter().map( |t| t.appearance).collect();

        let cands0 = find_pairs(&chars0);
        let cands1 = find_pairs(&chars1);
        for cand in &cands0 {
            if cands1.contains(&cand) {
                toreturn.push(*cand)
            }
        }

        let str0:String = chars0.iter().collect();
        let str1:String = chars1.iter().collect();
        println!("find_hline_candidates() unioned {}->{:?} and {}->{:?} for {:?}", str0, cands0, str1, cands1, toreturn);

        toreturn
    }    

}


pub fn find_pairs(tiles: &Vec<char>) -> Vec<usize> {
    let mut toreturn: Vec<usize> = Vec::new();
    for n in 0..tiles.len()-1 {
        // if tiles.get(n).unwrap() == tiles.get(n+1).unwrap() { // Remove these for part 1.
            toreturn.push(n);
        // }
    }
    toreturn
}


#[allow(dead_code)]
pub fn parse_input(content: &str) -> Vec<Pattern> {
    let split: Vec<&str> = content.split("\n\n").collect();
    let mut toreturn: Vec<Pattern> = Vec::new();
    for frag in split {
        if frag.trim().len() == 0 { continue; } 
        toreturn.push(Pattern::from_content(frag.trim()))
    }
    toreturn
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut num_cols_leftof:usize = 0;
    let mut num_lines_aboveof:usize = 0;
    
    let patterns = parse_input(input);
    for pattern in patterns {
        if let Some(hline) = pattern.find_hline_reflect(0) {
            num_lines_aboveof += hline+1

        } 
        if let Some(vline) = pattern.find_vline_reflect(0) {
            num_cols_leftof += vline+1
        } 

    }
    (num_cols_leftof + num_lines_aboveof * 100).try_into().unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut num_cols_leftof:usize = 0;
    let mut num_lines_aboveof:usize = 0;
    
    let patterns = parse_input(input);
    for pattern in patterns {
        if let Some(hline) = pattern.find_hline_reflect(1) {
            num_lines_aboveof += hline+1

        } 
        if let Some(vline) = pattern.find_vline_reflect(1) {
            num_cols_leftof += vline+1
        }
    }
    (num_cols_leftof + num_lines_aboveof * 100).try_into().unwrap()
}




#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d13::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_validate_splits() {
        let pbuf = input::get_input("2023_d13_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let patterns = parse_input(&content);

        // patt0 reflects vert @ col_nr 5 (4, zero-indexed)
        let patt0 = patterns.get(0).unwrap();

        // patt1 reflects horiz @ row_nr 4 (3, zero-indexed)
        let patt1 = patterns.get(1).unwrap();

        assert_eq!(0, patt1.line_equality(2-1, 7-1));
        assert_eq!(0, patt1.line_equality(3-1, 6-1));
        assert_eq!(0, patt1.line_equality(4-1, 5-1));

        for row_nr in 0..patt0.tiles.len()-1 {
            assert!(patt0.validate_hline_reflect(row_nr) > 0);
        }

        for row_nr in 0..patt1.tiles.len()-1 {
            if row_nr == 4-1 {
                let actual = patt1.validate_hline_reflect(row_nr);
                assert_eq!(0, actual, "Wrong value for row_nr=4-1 for patt=1.");
            } else {
                assert!(patt1.validate_hline_reflect(row_nr) > 0);
            }
        }

        for col_nr in 0..patt0.tiles.get(0).unwrap().len()-1 {
            if col_nr == 5-1 {
                assert_eq!(0, patt0.validate_vline_reflect(col_nr));
            } else {
                assert!(patt0.validate_vline_reflect(col_nr)> 0);
            }
        }

        for col_nr in 0..patt1.tiles.get(0).unwrap().len()-1 {
            assert!(patt1.validate_vline_reflect(col_nr)> 0);
        }

        
        
    }

    /*

    This test verified that vline and hline candidates were carefully selected based
    on any repeats in the first two lines.

    Performance did not end up mattering, and approach did not work for part 2 anyway.

    #[test]
    fn test_find_vert_candidates() {
        let pbuf = input::get_input("2023_d13_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let patterns = parse_input(&content);

        {
            // patt0 reflects vert @ col_nr 5 (4, zero-indexed)
            let patt0 = patterns.get(0).unwrap();
            let expected: Vec<usize> = Vec::from([4]);
            let actual = patt0.find_vline_candidates();
            assert_eq!(expected, actual);
        }

        {
            // patt1 reflects horiz @ row_nr 4 (3, zero-indexed)
            let patt1 = patterns.get(1).unwrap();
            let expected: Vec<usize> = Vec::from([0, 3]);
            let actual = patt1.find_hline_candidates();
            assert_eq!(expected, actual);
        }

    }
     */

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d13::tests --nocapture


        
        {
            let pbuf = input::get_input("2023_d13_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 405); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d13.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 35232); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d13_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 400); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d13.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert!(actual > 32842); // 32842 is too low.
            assert!(actual > 35662); // 35662 is too low
            assert_eq!(actual, 37982); // p2 skarp
        }
         
    }
}
