use rand::prelude::*;

use crate::utils::{utils::get_millis_now, Structures::DroneState};

pub fn get_drone_state(
    last_iter: &mut u64,
    drone: &mut DroneState,
    current_time: &mut u64,
    elapsed_millis: &mut u64,
    flawn_distance: &mut f64,
    &flight_start: &u64,
    &R: &f64,
) -> DroneState {
    let elapes_from_last_iter = get_millis_now() - *last_iter;
    *last_iter = get_millis_now();

    // read current speed
    drone.speed = get_curr_speed(elapes_from_last_iter, drone.speed);

    // read altitude will be usefull later
    drone.alt = get_altitude(drone.alt, drone.speed[2], elapes_from_last_iter);

    let DroneState {
        speed,
        alt: _,
        lat,
        lon,
        bearing,
    } = drone;

    // read current time
    *current_time = get_millis_now();

    let long_rad = lon.to_radians();
    let lat_rad = lat.to_radians();

    // calc elapsed time
    *elapsed_millis = *current_time - flight_start;

    // calculate flawn distance during interval
    let interv_flawn_distance = speed[0] as f64 * (elapes_from_last_iter as f64 / 1000.0);

    // calculate flawn distance
    // flawn_distance = speed[0] as f64 * (elapsed_millis / 1000) as f64;
    *flawn_distance += interv_flawn_distance;

    // calc angular distance
    let angular_distance = interv_flawn_distance / R;

    // calc radian bearing
    let radian_bearing = bearing.to_radians();

    // calc new latitude
    drone.lat = (((lat_rad.sin() * angular_distance.cos())
        + (lat_rad.cos() * angular_distance.sin() * radian_bearing.cos()))
    .asin())
    .to_degrees();

    // calc new longitude
    drone.lon = ((long_rad
        + (radian_bearing.sin() * angular_distance.sin() * lat_rad.cos())
            .atan2(angular_distance.cos() - (lat_rad.sin() * (drone.lat.to_radians()).sin())))
    .to_degrees()
        + 180.0)
        % 360.0
        - 180.0;

    return *drone;
}

pub fn allign_with_bearing(current: &f64, target: &f64, target_distance: &f32) -> f64 {
    let mut owned_bearing = current.to_owned();
    let mut owned_target = target.to_owned();

    let owned_target_dist = target_distance.to_owned();

    let mut turn_coef = 0.05;

    if normalize_bearing(&(owned_target - owned_bearing)) >= 45.0 && owned_target_dist <= 10.0 {
        turn_coef = 0.5
    }
    if normalize_bearing(&(owned_target - owned_bearing)) >= 90.0 && owned_target_dist <= 10.0 {
        turn_coef = 1.0
    }
    if normalize_bearing(&(owned_target - owned_bearing)) >= 180.0 && owned_target_dist <= 10.0 {
        turn_coef = 1.5
    }
    // if owned_bearing == 0.0 {
    //     owned_bearing = 360.0
    // }

    if owned_target == 0.0 {
        owned_target = 360.0
    }

    if owned_target == owned_bearing + 1.0 || owned_target == owned_bearing - 1.0 {
        return owned_bearing;
    };

    if owned_target <= (owned_bearing + 180.0) && owned_target > owned_bearing {
        owned_bearing += turn_coef;
        return owned_bearing;
    }

    owned_bearing -= turn_coef;
    return owned_bearing;
}

pub fn get_curr_speed(duration: u64, curr_speed: [f32; 3]) -> [f32; 3] {
    let mut speed = curr_speed;

    let accel_data = clear_gravity(get_accelerometer_data(speed));
    // println!("accel data : {:?}", accel_data);
    for i in 0..speed.len() {
        speed[i] += accel_data[i] * duration as f32 / 1000.0
    }
    return speed;
}

pub fn get_accelerometer_data(speed: [f32; 3]) -> [f32; 3] {
    let mut range = rand::thread_rng();
    let rng_begin = -5.0;
    let rng_end = 10.0;

    let speed_limit = 12.0;

    let x = range.gen_range(rng_begin..rng_end - (speed[0] / speed_limit)) as f32;
    let y = range.gen_range(-5.0 + rng_begin..rng_end - (speed[1] / speed_limit * 2.0)) as f32;
    let z =
        range.gen_range(-4.5 + rng_begin..rng_end - (speed[2] / speed_limit * 2.0)) as f32 + 9.8;

    return [x, y, z];
}

pub fn clear_gravity(accel: [f32; 3]) -> [f32; 3] {
    let mut tmp_accel = accel;
    tmp_accel[accel.len() - 1] -= 9.8;
    return tmp_accel;
}

pub fn get_altitude(curr_alt: f32, z_speed: f32, duration: u64) -> f32 {
    return curr_alt + (z_speed * (duration as f32 / 1000.0));
}

pub fn normalize_bearing(bearing: &f64) -> f64 {
    let owned_bearing = bearing.to_owned();

    if owned_bearing > 360.0 {
        return owned_bearing.to_owned() - 360.0;
    }

    if owned_bearing <= 0.0 {
        return owned_bearing + 360.0;
    }
    return owned_bearing;
}
