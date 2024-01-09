mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    add_to_waitlist();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
// }

// fn function2() -> io::Result {
// }

use rand::Rng;
fn r () {
    let secret_number = rand::thread_rng().gen_range(1..101);
}

mod front_of_house2;
// Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令する
pub use crate::front_of_house2::hosting2;