

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
    use crate::structs::I64Pair;
    use crate::input;
    use std::path::Path;

    #[test]
    fn d1_additive_complement() {
        let d1set = d1::i64_set_builder(D1_SAMPLE);
        assert!(d1set.contains(&675));

        match d1::find_summable_pairs(&d1set){
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
                match d1::find_summable_pairs(&skarp){
                    Some(I64Pair{x,y}) => {
                        println!("Found summable pairs: {} * {} -> {}\n", x, y, x*y);
                        assert!(x == 277);
                    },
                    None    => assert!(false, "Did not find summable pairs."),
                }
            }
        }
    }
}
