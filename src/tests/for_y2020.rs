

#[cfg(test)]
mod y2020tests {

    const D1_SAMPLE: &str = "1721
979
366
299
675
1456
";

    use crate::y2020::d1;
    use crate::structs::{I64Pair, I64Tri};
    use crate::input;

    #[test]
    fn d1_additive_complement() {
        let d1set = d1::i64_set_builder(D1_SAMPLE);
        assert!(d1set.contains(&675));

        match d1::find_summable_pairs(2020, &d1set){
            Some(I64Pair{x,y}) => {
                println!("Found summable pairs: {} {}\n", x, y);
                assert!(x == 299);
            },
            None    => assert!(false, "Did not find summable pairs."),
        }

        let fname_abs = match input::get_input("2020_d1.txt") {
            Err(why) => {
                assert!(false, "Could not open skarp input: {}", why);
                std::path::PathBuf::new()
            },
            Ok(s) => {
                s
            }
        };

        match input::readstring(&fname_abs) {
            Err(why) => assert!(false, "Could not open skarp input: {}", why),
            Ok(s) => {
                let skarp = d1::i64_set_builder(s.as_str());
                match d1::find_summable_pairs(2020, &skarp){
                    Some(I64Pair{x,y}) => {
                        println!("Found summable pairs: {} * {} -> {}\n", x, y, x*y);
                        assert!(x == 277);
                        assert!(y == 1743);
                    },
                    None    => assert!(false, "Did not find summable pairs."),
                }

                let skarp = d1::i64_set_builder(s.as_str());
                match d1::find_summable_tris(&skarp){
                    Some(I64Tri{x,y,z}) => {
                        println!("Found summable pairs: {} * {} * {} -> {}\n", x, y, z, x*y*z);
                        assert!(x == 262);
                        assert!(y == 691);
                        assert!(z == 1067);
                    },
                    None    => assert!(false, "Did not find summable pairs."),
                }
            }
        }
    }

    use crate::grids::walker::{GridWalker, Walking};

    #[test]
    fn d12_gridwalker() {
        let mut test_ship = GridWalker::new(0, 0);
        let test_moves_1 = "\
        F10
";
        let test_moves_2 = "\
N3
F7
R90
F11
";

        test_ship.ingest_moves(test_moves_1);
        assert_eq!(test_ship.manhattan_dist_origin(),10);

        test_ship.ingest_moves(test_moves_2);
        assert_eq!(test_ship.manhattan_dist_origin(),25);

        // Skarp
        let fname_abs = match input::get_input("2020_d12.txt") {
            Err(why) => {
                assert!(false, "Could not open skarp input: {}", why);
                std::path::PathBuf::new()
            },
            Ok(s) => {
                s
            }
        };

        match input::readstring(&fname_abs) {
            Err(why) => assert!(false, "Could not open skarp input: {}", why),
            Ok(s) => {
                let mut skarp_ship = GridWalker::new(0, 0);
                skarp_ship.ingest_moves(&s);
                assert_eq!(skarp_ship.manhattan_dist_origin(), 1533);
            }
        }
    }
}
