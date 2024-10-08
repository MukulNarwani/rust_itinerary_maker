pub use crate::model::State;
use std::io::*;
use std::io;
use strum::{EnumIter,IntoEnumIterator};

#[derive(EnumIter,Debug)]
enum StateMode{
    List,
    Add,
    Quit
}
//impl StateMode{
//    fn list_options(&self)->String{
//
//    }
//}
pub struct ControllerView{
    model: State,
}
impl ControllerView{
    pub fn new() -> Self {
        let mut app = ControllerView{ model : State::default()  };
        app.start();
        app

    }
    pub fn add_mode(&self){
        //TODO city builder
    }
    pub fn start(&mut self){
       loop{
                
                let response:String = self.input("What mode? ");
                //println!("{}",response.as_str());
                match response.as_str() {
                    "add" => {self.add_mode()},
                    "list" =>{ self.list_mode()},
                    "quit" =>{ println!("Have a good day!");break},
                    x => self.error(x,format!("Valid modes: {:?}",StateMode::iter().collect::<Vec<_>>()).to_string())
                }
        }
    }
    pub fn list_mode(&mut self){
        println!("{:?}",self.model.list_cities());
    }

    pub fn error(&mut self,input:&str,errormsg: String){
        println!("{}",errormsg);
        //println!("{}","list" == "list");
        //println!("You said:{}",input);
    }
    fn input(&mut self,prompt: &str) -> String {
         print!("{}", prompt);
         std::io::stdout().flush();

        let mut reply = String::new();
        io::stdin().read_line(&mut reply);
        reply.trim().to_lowercase().to_string()
        //println!("I got {}" ,reply.clone());
    }
}
