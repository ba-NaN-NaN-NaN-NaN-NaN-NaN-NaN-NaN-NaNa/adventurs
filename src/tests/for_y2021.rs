

#[cfg(test)]
mod y2021tests {

    use crate::input;
    
    #[test]
    fn d1_depth() {

        let fname_abs = match input::get_input("2021_d1.txt") {
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
            Ok(_) => {
                
            }
        };
    }
}
