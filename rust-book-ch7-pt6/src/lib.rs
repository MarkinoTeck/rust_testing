mod front_of_house;
/*
    = pub mod hosting {
        pub fn add_to_waitlist() {}
    }
*/

use crate::front_of_house::hosting as CoolHosting;

pub fn eat_at_restaurant() {
    CoolHosting::add_to_waitlist();
    CoolHosting::add_to_waitlist();
    CoolHosting::add_to_waitlist();
}