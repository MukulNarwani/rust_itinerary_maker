pub use crate::model::State;
use std::io::*;
use std::io;
use strum::{EnumIter,IntoEnumIterator};

#[derive(EnumIter,Debug)]
enum StateMode{
    List,
    Add,
    Main,
    Quit
}
//impl StateMode{
//    fn list_options(&self)->String{
//
//    }
//}
pub struct ControllerView{
    model: State,
    curr_state:StateMode,
}
impl ControllerView{
    pub fn new() -> Self {
        let mut app = ControllerView{ model : State::default() , curr_state: StateMode::Main  };
        app.tick();
        app

    }
    pub fn add_mode(&self){

    }
    pub fn list_mode(&mut self){
        
        println!("{:?}",self.model.list_cities());
        self.curr_state = StateMode::Main;
        self.tick();
    }
    pub fn main(&mut self){
        let response:String = self.input("What mode?");
        println!("{}",response.as_str());
        match response.as_str() {
            "Add" => {self.curr_state=StateMode::Add; self.tick()},
            "List" =>{self.curr_state=StateMode::List; self.tick()},
            "Quit" =>{self.curr_state=StateMode::Quit; self.tick()},
            x => self.error(x,format!("Valid modes: {:?}",StateMode::iter().collect::<Vec<_>>()).to_string())
        }
    }
    pub fn tick(&mut self){
        match self.curr_state{
            StateMode::Add => self.add_mode(),
            StateMode::List=> self.list_mode(),
            StateMode::Main => self.main(),
            StateMode::Quit => self.quit()
        }
    }
    fn quit(&mut self){
        println!("Have a good day!")
    }
    pub fn error(&mut self,input:&str,errormsg: String){
        println!("{}",errormsg);
        //println!("{}","list" == "list");
        //println!("You said:{}",input);
        self.curr_state=StateMode::Main;
        self.tick();
    }
    fn input(&mut self,prompt: &str) -> String {
         print!("{}", prompt);
         std::io::stdout().flush();

        let mut reply = String::new();
        io::stdin().read_line(&mut reply);
        reply.trim().to_string()
        //println!("I got {}" ,reply.clone());
    }
}
