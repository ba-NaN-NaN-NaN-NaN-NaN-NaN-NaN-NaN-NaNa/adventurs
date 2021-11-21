

#[cfg(test)]
mod tests {
    use crate::models::user_model::print_user_model as log_user_model;
    #[test]
    fn it_works() {
        log_user_model();
        assert_eq!(2 + 2, 4);
    }
}
