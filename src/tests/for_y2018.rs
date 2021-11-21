

#[cfg(test)]
mod tests {
    use crate::y2018::d1::freq_summer;
    #[test]
    fn it_works() {
        assert_eq!(freq_summer(0, "+1, +1, +1"),  3);
        assert_eq!(freq_summer(0, "+1, +1, -2 "),  0);
        assert_eq!(freq_summer(0, "-1, -2, -3 "), -6);
    }
}
