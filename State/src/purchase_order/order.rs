/* 
 * Author: lminozzo
 *
 */
use super::states::*;

pub trait OrderOperations{
    fn get_name(&self)-> &'static str;
    fn approve(&self, order: &mut InnerOrder)-> Option<Box<OrderOperations>>;
    fn reprove(&self, order: &mut InnerOrder)-> Option<Box<OrderOperations>>;
    fn buy(&self, order: &mut InnerOrder)-> Option<Box<OrderOperations>>;
    fn close(&self, order: &mut InnerOrder)-> Option<Box<OrderOperations>>;
}

pub struct Order {
    inner_order: InnerOrder,
    state: Box<OrderOperations>,
}

pub struct InnerOrder{
    log: String,
}

impl Order {
    pub fn create() -> Order {
        Order { inner_order:InnerOrder{log: String::from("")}, state: Box::new(FirstApprover) }
    }
    pub fn approve(&mut self) {
        match self.state.approve(& mut self.inner_order) {
            Some(state) => self.state = state,
            _ => {}
        }

    }
    pub fn reprove(&mut self) {
        match self.state.reprove(& mut self.inner_order) {
            Some(state) => self.state = state,
            _ => {}
        }
    }
    pub fn buy(&mut self) {
        match self.state.buy(& mut self.inner_order) {
            Some(state) => self.state = state,
            _ => {}
        }
    }
    pub fn close(&mut self) {
        match self.state.close(& mut self.inner_order) {
            Some(state) => self.state = state,
            _ => {}
        }
    }

    pub fn get_log(&self)->String{
        self.inner_order.show_log()
    }
}

impl InnerOrder {
    pub fn show_log(&self) -> String {
        self.log.clone()
    }

    pub fn set_log(&mut self, log: &str, state:&str, operation: &str) {
        self.log.push_str("[");
        self.log.push_str(state);
        self.log.push_str("][");
        self.log.push_str(operation);
        self.log.push_str("]");
        self.log.push_str(log);
        self.log.push_str("\n");
    }
}