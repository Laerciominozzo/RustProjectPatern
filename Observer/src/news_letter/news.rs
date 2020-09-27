/*
 * Author: lminozzo
 *
 */



pub trait NotifyReciver{
    fn receiver(&self, last_news: &str);
}

pub struct News<'a,'b>{
    last_news: &'a str,
    pub notifier :NewsNotifier<'b>
}

impl<'a,'b> News<'a,'b> {
    pub fn create () ->News<'a, 'b>{
        News{
            last_news:"",
            notifier: NewsNotifier{
                receivers: Vec::new()
            }
        }
    }
    pub fn add_news(&mut self,title : &'a str){
        self.last_news = title;
        println!("Publicando a noticia: {}", self.last_news);
        self.notifier.notify(self.last_news);
    }
}

pub struct NewsNotifier<'b>{
    receivers:Vec<&'b NotifyReciver>
}

impl<'t> NewsNotifier<'t> {
    pub fn register(&mut self , receiver : &'t NotifyReciver) -> u32{
        self.receivers.push(receiver);
        self.receivers.len() as u32 - 1
    }

    pub fn unregister(& mut self, handler:u32){
        let index = handler as usize;
        if index < self.receivers.len() {
            self.receivers.remove(index);
        }
    }

    pub fn notify(&self, title : &str){
        for receiver in &self.receivers {
            receiver.receiver(title);
        }
    }
}