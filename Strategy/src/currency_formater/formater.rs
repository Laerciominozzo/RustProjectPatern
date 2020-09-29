/* 
 * Author: lminozzo
 *
 */

pub trait TraitFormater{
    fn format(&self, number:f32) -> String;
}

pub struct Formater{
    formater: Box<dyn TraitFormater>
}

impl Formater{
    pub fn create( formater : Box<dyn TraitFormater>)->Formater{
        Formater{
            formater
        }
    }

    pub fn format(&self, number: f32) -> String{
        self.formater.format(number)
    }
}


pub struct NormalNumber;

impl TraitFormater for NormalNumber {
    fn format(&self, number: f32) -> String{
       format!("{:.2}",number)
    }
}

pub struct RealCurrency;

impl TraitFormater for RealCurrency {
    fn format(&self, number: f32) -> String {
        format!("R${:.2}",number).replace(".",",")
    }
}