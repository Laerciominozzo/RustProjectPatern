mod currency_formater;

use currency_formater::{NormalNumber, RealCurrency};
use crate::currency_formater::Formater;

fn main() {
    let normal_number = Box::new(NormalNumber{});

    let formater = Formater::create(normal_number);

    println!("{}", formater.format(1.5));

    let real_currency = Box::new(RealCurrency{});

    let formater = Formater::create(real_currency);

    println!("{}", formater.format(1.5));
}
