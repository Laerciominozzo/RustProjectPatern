/*
 * Author: lminozzo
 *
 */

use crate::purchase_order::order::Order;

mod purchase_order;
fn main() {
    println!("[ Estado ][ Operação ] Resultado => Complemento");
    println!("============================================");

    let mut  order :Order = Order::create();

    order.approve();
    order.approve();
    order.buy();
    order.approve();

    print!("{}",order.get_log());
    println!("============================================");

    order = Order::create();

    order.buy();
    order.approve();
    order.reprove();
    order.buy();
    order.close();

    print!("{}",order.get_log());
    println!("============================================");
    order = Order::create();

    order.approve();
    order.reprove();
    order.close();
    order.buy();
    order.close();

    print!("{}",order.get_log());
    println!("============================================");
    order = Order::create();

    order.approve();
    order.approve();
    order.reprove();
    order.buy();
    order.close();

    print!("{}",order.get_log());
    println!("============================================");

}
