#[derive(Debug, Clone, Copy)]
pub struct DroneState {
    pub lon: f64,
    pub lat: f64,
    pub alt: f32,
    pub bearing: f64,
    pub speed: [f32; 3],
}
pub struct Coordinates {
    pub lon: f64,
    pub lat: f64,
}
