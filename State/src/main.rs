/*
 * Author: lminozzo
 *
 */

use crate::purchase_order::order::Order;

mod purchase_order;
fn main() {

    let mut  order :Order = Order::create();

    order.approve();
    order.approve();
    order.buy();
    order.approve();

    print!("{}",order.get_log());

}
