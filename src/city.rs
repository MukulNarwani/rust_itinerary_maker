use ndarray::{array, Array2};
use std::cmp;
//pub mod city;
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Location {
    pub name: String,
    pub coord: (u8, u8),
}
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Activity {
    pub location: Location,
    price: u8,
    id: u8,
}
#[derive(Clone, PartialEq, Debug)]
pub struct City { // impl Location for City!!!
    activities_clustered: Array2<u8>,
    activities: Vec<Activity>, //HASHMAP??
    pub location: Location,
}
pub fn get_max_values(this : (u8,u8),other : (u8,u8)) -> (u8,u8){
    let i = cmp::max(this.0, other.0);
    let j = cmp::max(this.1, other.1);
    (i,j)
}
//impl Location{
//    pub fn new(coord:(u8,u8)) -> Self{
//        Location{name:"".to_string(),coord:(coord.0,coord.1)}
//    }
//}
impl City {
    pub fn new(location: Location) -> City {
        //TODO
        City {
            activities: vec![],
            location,
            activities_clustered: array![[], []],
        }
    }
    pub fn add_activity(&mut self, activity: Activity) {
        // TODO Push activity if not error
        self.activities.push(activity)
    }
    pub fn remove_activity(&mut self, id: u8) {
        //TODO
        self.activities.remove(id.try_into().unwrap());
    }
    pub fn list_activities(&self) -> &Vec<Activity> {
        // TODO Keep or remove? pretty print activities
        &self.activities
    }
    pub fn update_activity(&mut self, id: u8, activity: Activity) {
        // TODO
        self.activities.insert(id.try_into().unwrap(), activity);
        self.activities.remove((id + 1).try_into().unwrap());
    }
}
