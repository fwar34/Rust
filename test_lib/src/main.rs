extern crate test_lib;

use a::series::of;

use Color::{Red, Green};
//use Color::*;

fn main() {
    test_lib::client::connect();
    of::modes();

    let _red = Red;
    let _blue = Color::Blue;
    let _green = Green;
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn modes() {

            }
        }
    }
}

enum Color {
    Red,
    Green,
    Blue,
}
