

#[cfg(test)]
mod y2018tests {
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
