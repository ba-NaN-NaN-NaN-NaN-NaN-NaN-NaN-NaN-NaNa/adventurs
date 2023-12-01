

#[cfg(test)]
mod y2022tests {
    mod d1 {
        use crate::input;
        use crate::y2022::d1;

        #[test]
        fn d1() {
            d1p1();
            d1p2();
        }

        #[test]
        fn d1p1() {
            {
                let pbuf = input::get_input("2022_d1.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = d1::part1(&content);
                assert_eq!(actual, 71934); // part1 skarp
            }
        }

        #[test]
        fn d1p2() {
            {
                let pbuf = input::get_input("2022_d1.txt").unwrap();
                let content = input::readstring(&pbuf).unwrap();
                let actual = d1::part2(&content);
                assert_eq!(actual, 211447); // part2 skarp
            }
        }
    }
}
