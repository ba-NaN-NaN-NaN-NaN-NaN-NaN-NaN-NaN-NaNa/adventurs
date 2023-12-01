

#[cfg(test)]
mod y2023tests {

    mod d1 {
        use crate::input;
        use crate::y2023::d1::*;

        #[test]
        fn test_despell() {
            assert_eq!(despell("eightwothree".to_string()), "8wo3".to_string()); 
            assert_eq!(despell("jhctmxconelfkgmprnfourseven8twofkjvlvnjgd".to_string()), "1lfkgmprnfourseven82".to_string());
        }

        #[test]
        fn d1() {
            d1p1();
            d1p2();
        }

        #[test]
        fn d1p1() {
            {
                let pbuf = input::get_input("2023_d1_sample.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = part1(&content);
                assert_eq!(actual, 142); // p1 sample
            }
            {
                let pbuf = input::get_input("2023_d1.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = part1(&content);
                assert_eq!(actual, 55090); // part1 skarp
            }
        }

        #[test]
        fn d1p2() {
            {
                let pbuf = input::get_input("2023_d1_sample_p2.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = part2(&content);
                assert_eq!(actual, 281); // p2 sample
            }
            {
                let pbuf = input::get_input("2023_d1.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = part2(&content);
                assert_eq!(actual, 54845); // part2 skarp
            }
        }
    }
}
