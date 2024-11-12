#![allow(unused)]
mod model;
mod city;
mod controller_view;
use crate::model::*;
use plotters::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use controller_view::*;


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

    let mut rng = thread_rng();
    let side = Uniform::new(0, 5);

    let mut cities: Vec<City> = vec![];
    let city_names = ["Bristol","Cambridge","Oxford",
                    "London","Edinburgh","Birmingham",
                    "Leeds","Sheffield","Abu Dhabi","Boston"];
    // sample between 1 and 10 points
    for city_name in &city_names {
        // sample a point from the square with sides -10 - 10 in two dimensions
        let (x, y) = (rng.sample(side) as i32, rng.sample(side) as i32);
        let tmp_city1: City = City::new(Location {
            name: city_name.to_string(),
            coord: (x, y),
        });
        let tmp_city2: City = City::new(Location {
            name: format!("{} + 5",city_name),
            coord: (x + 10, y + 10),
        });
        model.add_city(tmp_city1);
        model.add_city(tmp_city2);
    }
    let cluster = model.tmp_cluster_activities();
   let cli = ControllerView::new();
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
}

