/* 
 * Author: lminozzo
 *
 */

use super::news::NotifyReciver;

pub struct Person{
    reference: String,

}

impl Person {
    pub fn create(reference: String) -> Person{
        Person{reference}
    }


}
impl NotifyReciver for Person{


    fn receiver(&self, last_news: &str) {
        println!(" {} está lendo a noticia: {}",self.reference, last_news);
    }
}