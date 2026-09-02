#[derive(PartialEq, Eq, Debug)]
pub enum CruiseControlPositionEnum {
    VisibleAheadTarget,
    VisibleBehindTarget,
    VisibleOnTarget,
    NotVisible,
}

pub struct CarConfig {
    pub position_m: f32,
    pub current_speed_ms: f32, //meters/second
    pub cruise_speed_ms: f32,  //meters/second
    pub min_gap_m: f32,
    pub time_headway_sec: f32,
    pub acceleration_mssq: f32, // meters/second^2
    pub braking_mssq: f32,      // meters/second^2
}

#[derive(Clone, Copy, Debug)]
pub struct RadarReading {
    pub distance_m: f32,
    pub relative_speed: Option<f32>,
    pub distance_from_target_s: Option<f32>
}