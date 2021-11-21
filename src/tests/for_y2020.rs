

#[cfg(test)]
mod y2020tests {

    const D1_SAMPLE: &str = "1721
979
366
299
675
1456
";

    use crate::y2020::d1::i64_set_builder;
    #[test]
    fn d1_additive_complement() {
        

        let d1set = i64_set_builder(D1_SAMPLE);
        assert!(d1set.contains(&675));
    }

}
