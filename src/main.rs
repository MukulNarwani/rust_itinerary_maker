#![allow(unused)]
use ndarray::{array, range, Array,Array2,Ix2};
use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::{
    arch::aarch64::float32x2_t,
    borrow::BorrowMut,
    collections::{btree_map::Values, HashMap},
    env::VarError,
    error::Error,
    iter::Map,
};
mod city;
//mod location;
//mod activity;
pub use city::*;

#[derive(Clone, Default, Debug)]
struct CityCache {
    city_cache: HashMap<String, City>, // make in its own struct
}
#[derive(Clone, Default, Debug)]
struct State {
    active_city: Option<City>,
    cache: Option<CityCache>,
}
//impl Iterator for CityCache{
//    type Item = City;
//    fn next(&mut self) -> Option<Self::Item>{
//
//    }
//}
impl CityCache {
    pub fn new() -> Self {
        CityCache {
            city_cache: HashMap::new(),
        }
    }
    pub fn insert(&mut self, city: &City) {
        self.city_cache
            .insert(city.location.name.to_string(), city.clone());
    }
    pub fn get_city(&mut self, city: &str) -> Result<&mut City, String> {
        match self.city_cache.get_mut(city) {
            Some(y) => Ok(y),
            None => Err("City doesn't exist".to_string()),
        }
    }
    pub fn add_city(&mut self, city: City) {
        // Result<(), String> {
        let name: &str = &city.location.name;
        self.city_cache.insert(name.to_string(), city); // TODO decide whether to enforce capacity or not
    }
    fn construct_graph(&self) -> Array<String,Ix2> {
        let cache_len =  self.city_cache.len();
        let mut coords_list :Vec<(&str,i32,i32)> = Vec::with_capacity(cache_len);
        //TODO panics
        let mut max_coords:(i32,i32) = (0,0); // Iterates twice which seems wastefull
        for (name,city) in self.city_cache.iter() {
            let cur_coords = city.location.coord;
            coords_list.push((name,cur_coords.0,cur_coords.1));
            max_coords = get_max_values(max_coords, cur_coords);
        }
        let mut cities_clustered: Array<String,Ix2>= Array::default(((max_coords.0 + 1) as usize,(max_coords.1 +1) as usize));
        for (city,i,j) in coords_list {
            //println!("{}, {}",i,j);
           cities_clustered[[i as usize,j as usize]] = city.to_string() ;
        };
        cities_clustered
    } 
}
impl State {
    pub fn set_active_city(&mut self, city: &City) {
        self.active_city = Some(city.clone());
    }
    pub fn save_city(&mut self) -> Result<(), String> {
        //TODO checks and balances
        self.cache
            .get_or_insert(CityCache::new())
            .insert(&self.active_city.clone().ok_or("No active city")?);
        Ok(())
    }
    pub fn list_cities(&self) -> Result<&CityCache, String> {
        //TODO list only city names?
        match &self.cache {
            Some(x) => Ok(x),
            None => Err("No cache".to_string()),
        }
    }
    pub fn add_city(&mut self, city: City) {
        //TODO make add city return a result
        if self.active_city.is_none() {
            self.set_active_city(&city);
        };
        let cache: &mut CityCache = self.cache.get_or_insert(CityCache::new());

        cache.add_city(city);
    }
    pub fn remove_city(&mut self, city: City) -> Result<City, String> {
        match &mut self.cache {
            Some(x) => {
                let res: Option<City> = x.city_cache.remove(&city.location.name);
                res.ok_or(format!("City {} does not exist", &city.location.name))
            }
            None => Err("No Cities in Cache".to_string()),
        }
    }
    pub fn get_points(&self) -> Vec<(i32, i32)> {
        self.cache
            .clone()
            .unwrap_or_default()
            .city_cache
            .values()
            .map(|x| x.location.coord)
            .collect::<Vec<_>>()
    }


    
    pub fn tmp_cluster_activities(&mut self) {}
}

fn plot(state: &State) {
    let data1: &[(i32, i32)] = &state.get_points();
    let root_area = BitMapBackend::new("2.6.png", (1000, 1000)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Cities scattered", ("sans-serif", 40))
        .build_cartesian_2d(0..20, 0..20)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        data1
            .iter()
            .map(|point| TriangleMarker::new(*point, 5, BLUE)),
    )
    .unwrap();
    //const data1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
}
fn main() {
    let mut model: State = Default::default();
    let city2: City = City::new(Location {
        name: "Glasgow".to_string(),
        coord: (1, 1),
    });
    let city: City = City::new(Location {
        name: "NZ".to_string(),
        coord: (2, 2),
    });
    model.set_active_city(&city2);
    model.add_city(city);
    model.save_city();
    //println!("{:?}", x);
    //model.add_city(City { activities_clustered: (), activities: (), location: () });

    let mut rng = thread_rng();
    let side = Uniform::new(0, 5);
    //let mut points:Vec<(f32,f32)>=vec![];
    let mut cities: Vec<City> = vec![];

    // sample between 1 and 10 points
    for i in range(0.0, 10.0, 1.0) {
        // sample a point from the square with sides -10 - 10 in two dimensions
        let (x, y) = (rng.sample(side) as i32, rng.sample(side) as i32);
        let tmp_city1: City = City::new(Location {
            name: format!("{}", i).to_string(),
            coord: (x, y),
        });
        let tmp_city2: City = City::new(Location {
            name: format!("{} * 5", i).to_string(),
            coord: (x + 10, y + 10),
        });
        model.add_city(tmp_city1);
        model.add_city(tmp_city2);
    }

    //println!("{:?}",&model.list_cities());
    println!("{:?}",model.cache.expect("lol").construct_graph());
    //println!("{:?}",&model.cache.unwrap().city_cache.keys());
}
