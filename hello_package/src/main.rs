// in src/main.rs
use crate::front_of_house::hosting;
mod front_of_house;
// 填空并修复错误
fn main() {
    assert_eq!(hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
