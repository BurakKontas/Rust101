use crate::garden::vegetables::Asparagus;
use restaurant::front_of_house::hosting;
use restaurant::front_of_house::serving;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    hosting::add_to_waitlist();
    hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}