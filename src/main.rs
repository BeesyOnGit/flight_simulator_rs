mod utils;

use std::{fs::File, io::Write, thread, time::Duration};

use utils::{
    distance::{get_bearing_from_target, get_distance_from_target},
    location::{allign_with_bearing, get_curr_speed, get_drone_state, normalize_bearing},
    utils::{create_kml, get_millis_now},
    Structures::{Coordinates, DroneState},
};

#[tokio::main]
async fn main() {
    // contants declaration

    // earth radius
    let R = 6_378_137.0;
    // let pi = PI;

    // init drone state at T0
    let mut drone = DroneState {
        lon: 3.351990,
        lat: 36.743220,
        alt: 17.9,
        speed: [0.0, 0.0, 0.0],
        bearing: 0.0,
    };

    // define flight time in minutes
    let flight_time_min = 1;

    // define interval at which we "read" calculate data
    let duration_millis: u64 = 10;

    // convert flight time to milli seconds
    let flight_time_millis = flight_time_min * 60 * 1000;

    // get time at which flight starts
    let flight_start = get_millis_now();

    // init the current time reading var
    let mut current_time = flight_start;

    // calculate elapsed time from start
    let mut elapsed_millis = current_time - flight_start;

    // init distance flawn
    let mut flawn_distance = 0.0;

    // init last don iteration
    let mut last_iter = get_millis_now();

    let mut distance_from_target = 0.0;

    println!("flight started");
    let mut recording = String::new();

    // begin loop
    loop {
        // time limit condition
        // if elapsed_millis >= flight_time_millis {
        //     break;
        // }

        // sleep condition
        thread::sleep(Duration::from_millis(duration_millis));
        // read drone state at X time
        drone = get_drone_state(
            &mut last_iter,
            &mut drone,
            &mut current_time,
            &mut elapsed_millis,
            &mut flawn_distance,
            &flight_start,
            &R,
        );

        // setup target coordinates
        let target = Coordinates {
            lon: 3.024863,
            lat: 36.761313,
        };

        // calculat straight distance from the target
        distance_from_target = get_distance_from_target(
            &Coordinates {
                lon: drone.lon,
                lat: drone.lat,
            },
            &target,
            &R,
        );

        // calculate angle of the target
        let bearing = get_bearing_from_target(
            &Coordinates {
                lon: drone.lon,
                lat: drone.lat,
            },
            &target,
        );

        // adapt drone angle to match target
        let allignment = allign_with_bearing(&drone.bearing, &bearing, &distance_from_target);

        // update drone state
        drone.bearing = normalize_bearing(&allignment);

        // update flight record
        recording.push_str(&format!("{},{},{}\n", &drone.lon, &drone.lat, &drone.alt));

        // TODO decrease altitude of drone to reach target

        // condition to stop flight
        if distance_from_target <= 0.5 {
            break;
        }
    }

    let mut file = File::create("./records.kml").unwrap();
    file.write_all(create_kml(recording).as_bytes()).unwrap();

    println!(
        "\n \n drone final state : {:?} \n flight time : {} min \n flawn distance : {} m \n from target : {}",
        drone, ((last_iter-flight_start)/(1000*60)), flawn_distance, distance_from_target
    )
}
