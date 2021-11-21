
mod adder;
mod models;
mod tests;
mod y2018;

use crate::models::user_model::print_user_model as log_user_model;

fn main() {
    log_user_model();
    println!("Hello, world {}!", adder::add_two(3));
}
