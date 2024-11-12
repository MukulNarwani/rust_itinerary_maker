pub use crate::model::State;
use crate::City;
use std::io;
use std::io::*;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug)]
enum StateMode {
    List,
    Add,
    Quit,
}
pub struct ControllerView {
    model: State,
}

impl ControllerView {
    pub fn new() -> Self {
        let mut app = ControllerView {
            model: State::default(),
        };
        app.start();
        app
    }
    pub fn add_mode(&self) {
        //TODO city builder
        loop{
            let object = input("Activity, City, or Country? ");
            match object.as_str() {
                "activity" => {println!("activity");break;},
                "city" => {println!("city");break;},
                "country" => {println!("country");break;},
                x => {
                    println!("Unrecognized {:?}", x);
                }
            }
        }
    }
    pub fn start(&mut self) {
        loop {
            let response: String = input("what mode? ");
            match response.as_str() {
                "add" => self.add_mode(),
                "list" => self.list_mode(),
                "quit" => {
                    println!("have a good day!");
                    break;
                }
                x => self.error(
                    x,
                    format!("valid modes: {:?}", StateMode::iter().collect::<Vec<_>>()).to_string(),
                ),
            }
        }
    }
    pub fn list_mode(&mut self) {
        println!("{:?}", self.model.list_cities());
    }

    pub fn error(&mut self, input: &str, errormsg: String) {
        println!("{}", errormsg);
    }
}
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush();

    let mut reply = String::new();
    io::stdin().read_line(&mut reply);
    reply.trim().to_lowercase().to_string()
}

// TODO Delegate activity building to city? 
//pub fn activity_builder() -> Result<City,String> {
//   Location = input() //TODO validity check
//   price= input() //TODO validity check
//   = input() //TODO validity check
//}
//pub fn city_builder() -> Result<City,String> {
//    // TODO use reverse geocoding to find name from coordinates
//   let location = input("What is the name of the place? ");
//   let coord = input("What are the coordinates? ");
//
//}
//pub fn country_builder() -> Result<City,String> {}
