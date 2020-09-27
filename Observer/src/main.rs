/*
 * Author: lminozzo
 *
 */

mod news_letter;

use news_letter::*;
fn main() {

    let mut news = News::create();

    let mut person1 = Person::create("Leocadia".to_string());
    let mut person2 = Person::create("Robson".to_string());

    let person1Handler = news.notifier.register(&person1);


    let person2Handler = news.notifier.register(&person2);


    news.add_news("Acidente aereo registrado ontem!");

    news.notifier.unregister(person1Handler);

    news.add_news("Politico ladrão se converte paz mundial.")

}
