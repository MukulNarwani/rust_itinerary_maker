#![allow(unused)]
use kmedoids::*;
use ndarray::{array, range,ArrayBase, Array,Array2,Ix2};
use std::{
    arch::aarch64::float32x2_t,
    borrow::BorrowMut,
    collections::{btree_map::Values, HashMap},
env::VarError,
    error::Error,
    iter::Map,
};
//mod city;
//mod location;
//mod activity;
pub use crate::city::*;

#[derive(Clone, Default, Debug)]
struct CityCache {
    // Should be Coords : City
    city_cache: HashMap<String, City>, 
}
#[derive(Clone, Default, Debug)]
pub struct State {
    active_city: Option<City>,
    cache: Option<CityCache>,
}
//impl Iterator for CityCache{
//    type Item = City;
//    fn next(&mut self) -> Option<Self::Item>{
//
//    }
//}
//pub trait Model {
//    fn list_cities(
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
    pub fn list_cities(&self) -> Result<Vec<String>, String> {
        //TODO list only city names?
        match &self.cache {
            Some(x) => Ok(x.city_cache.clone().into_keys().collect()),
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

    
    pub fn tmp_cluster_activities(&mut self) -> Array<String,Ix2> {
        self.cache.clone().expect("lol").construct_graph()
        //let dissim= kmedoids::ArrayAdapter(cluster);
        //ndarray::arr2(&[[0,1,2,3],[1,0,4,5],[2,4,0,6],[3,5,6,0]]);
        //let data :  ndarray::ArrayBase<A, ndarray::dimension::dim::Dim<[usize; 2]>> = 
        //let mut meds = random_initialization(4, 2, &mut rand::thread_rng());
        //let (loss, assi, n_iter, n_swap): (f64, _, _, _) = fasterpam(&data, &mut meds, 100); 

    }
}

