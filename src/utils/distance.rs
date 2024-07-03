use super::Structures::Coordinates;

// calculating the distance betwen two coordinates using haversine equation
pub fn get_distance_from_target(coordinates: &Coordinates, target: &Coordinates, R: &f64) -> f32 {
    let Coordinates { lon, lat } = coordinates;
    let Coordinates {
        lon: lon_targ,
        lat: lat_targ,
    } = target;

    let delta_lat = lat_targ.to_radians() - lat.to_radians();
    let delta_lon = lon_targ.to_radians() - lon.to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat.to_radians().cos() * lat_targ.to_radians().cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    return (R * c) as f32;
}

pub fn get_bearing_from_target(coordinates: &Coordinates, target: &Coordinates) -> f64 {
    let Coordinates { lon, lat } = coordinates;
    let Coordinates {
        lon: lon_targ,
        lat: lat_targ,
    } = target;

    let delta_lon = lon_targ.to_radians() - lon.to_radians();

    // Calculate the bearing
    let y = delta_lon.sin() * lat_targ.to_radians().cos();
    let x = lat.to_radians().cos() * lat_targ.to_radians().sin()
        - lat.to_radians().sin() * lat_targ.to_radians().cos() * delta_lon.cos();
    let mut bearing_rad = y.atan2(x);

    // Convert bearing from radians to degrees
    let mut bearing_deg = bearing_rad.to_degrees();

    // Normalize the bearing to 0-360 degrees
    if bearing_deg < 0.0 {
        bearing_deg += 360.0;
    }

    return bearing_deg;
}
