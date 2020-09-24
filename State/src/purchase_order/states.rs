/* 
 * Author: lminozzo
 *
 */
use super::order::{InnerOrder,OrderOperations};

pub struct FirstApprover;
pub struct SecondApprover;
pub struct Purchase;
pub struct Closed;
pub struct Concluded;

impl OrderOperations for FirstApprover {

    fn get_name(&self) -> &'static str {
        "Em aprovação do 1º analista"
    }

    fn approve(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem aprovada pelo promeiro analista.",
                      self.get_name(),"Aprovar");
        Some(Box::new(SecondApprover))
    }

    fn reprove(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem Reprovada.",
                      self.get_name(),"Reprovar");
        Some(Box::new(Closed))
    }

    fn buy(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Erro => A compra não pode ser feita antes da aprovação.",
                      self.get_name(),"Comprar");
        None
    }

    fn close(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem fechada.",
                      self.get_name(),"Fechar");
        Some(Box::new(Closed))
    }
}

impl OrderOperations for SecondApprover {
    fn get_name(&self) -> &'static str {
        "Em aprovação do 2º analista"
    }

    fn approve(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem aprovada pelo segundo analista.",
                      self.get_name(),"Aprovar");
        Some(Box::new(Purchase))
    }

    fn reprove(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem reprovada.",
                      self.get_name(),"Reprovar");
        Some(Box::new(FirstApprover))
    }

    fn buy(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A compra não pode ser feita antes da aprovação.",
                      self.get_name(),"Comprar");
        None
    }

    fn close(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem fechada.",
                      self.get_name(),"Fechar");
        Some(Box::new(Closed))
    }
}

impl OrderOperations for Purchase {
    fn get_name(&self) -> &'static str {
        "Em compra"
    }

    fn approve(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A ordem não pode ser aprovada durante a compra",
                      self.get_name(),"Aprovar");
        None
    }

    fn reprove(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A ordem não pode ser reprovada durante a compra",
                      self.get_name(),"Reprovar");
        None
    }

    fn buy(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Compra realizada com sucesso.",
                      self.get_name(),"Comprar");
        Some(Box::new(Concluded))
    }

    fn close(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("OK => Ordem fechada.",
                      self.get_name(),"Fechar");
        Some(Box::new(Closed))
    }
}

impl OrderOperations for Closed {
    fn get_name(&self) -> &'static str {
        "Fechada"
    }

    fn approve(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está fechada.",
                      self.get_name(),"Aprovar");
        None
    }

    fn reprove(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está fechada.",
                      self.get_name(),"Reprovar");
        None
    }

    fn buy(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está fechada.",
                      self.get_name(),"Comprar");
        None
    }

    fn close(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está fechada",
                      self.get_name(),"Fechar");
        None
    }
}

impl OrderOperations for Concluded {
    fn get_name(&self) -> &'static str {
        "Concluida"
    }

    fn approve(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está concluida.",
                      self.get_name(),"Aprovar");
        None
    }

    fn reprove(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está concluida.",
                      self.get_name(),"Reprovar");
        None
    }

    fn buy(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está concluida.",
                      self.get_name(),"Comprar");
        None
    }

    fn close(&self, order: &mut InnerOrder) -> Option<Box<dyn OrderOperations>> {
        order.set_log("Err => A está concluida",
                      self.get_name(),"Fechar");
        None
    }
}

