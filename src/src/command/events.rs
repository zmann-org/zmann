use simconnect_sdk::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
#[allow(dead_code)]
pub struct AircraftData {
    #[simconnect(name = "TITLE")]
    aircraft_title: String,
    #[simconnect(name = "CATEGORY")]
    aircraft_category: String,
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    aircraft_lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    aircraft_lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
    aircraft_alt: f64,
    #[simconnect(name = "PLANE ALT ABOVE GROUND", unit = "feet")]
    aircraft_alt_above_ground: f64,
    #[simconnect(name = "VERTICAL SPEED", unit = "ft/min")]
    aircraft_vertical_speed: f64,
    #[simconnect(name = "G Force", unit = "GForce")]
    pub g_force: f64,
    #[simconnect(name = "CAMERA STATE")]
    pub view: f64
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
#[allow(dead_code)]
pub struct SimData {
    #[simconnect(name = "SIM ON GROUND")]
    sim_on_ground: bool,
    #[simconnect(name = "SIM DISABLED")]
    pub active: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SimConnectEvent {
    AircraftData(AircraftData),
    SimData(SimData),
}

#[allow(dead_code)]
pub enum TauriEvent {}