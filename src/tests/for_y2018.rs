

#[cfg(test)]
mod y2018tests {
    use crate::y2018::d1::freq_summer;
    #[test]
    fn d1_frequency_summer() {
        assert_eq!(freq_summer(0, "+1, +1, +1"),  3);
        assert_eq!(freq_summer(0, "+1, +1, -2 "),  0);
        assert_eq!(freq_summer(0, "-1, -2, -3 "), -6);
    }

    use crate::y2018::d5::reducer;

    #[test]
    fn d5_reducer() {
        assert_eq!(reducer("ab".as_bytes().to_vec()), "ab".as_bytes().to_vec());
        assert_eq!(reducer("aa".as_bytes().to_vec()), "aa".as_bytes().to_vec());
        assert_eq!(reducer("aA".as_bytes().to_vec()), "".as_bytes().to_vec());

        assert_eq!(reducer("dabAcCaCBAcCcaDA".as_bytes().to_vec()), "dabAaCBAcCcaDA".as_bytes().to_vec());
        assert_eq!(reducer("dabAaCBAcCcaDA".as_bytes().to_vec()), "dabCBAcCcaDA".as_bytes().to_vec());
        assert_eq!(reducer("dabCBAcCcaDA".as_bytes().to_vec()), "dabCBAcaDA".as_bytes().to_vec());
        assert_eq!(reducer("dabCBAcaDA".as_bytes().to_vec()), "dabCBAcaDA".as_bytes().to_vec());
    }
}
