use crate::*;
use std::collections::VecDeque;

mod service_structs;
#[cfg(test)]
mod tests;

pub use service_structs::{CarConfig, CruiseControlPositionEnum, RadarReading};

#[derive(Clone)]
pub struct Car {
    position_m: f32,
    current_speed_ms: f32, //meters/second
    cruise_speed_ms: f32,  //meters/second
    min_gap_m: f32,
    time_headway_sec: f32,
    acceleration_mssq: f32, // meters/second^2
    braking_mssq: f32,      // meters/second^2
    radar_readings: VecDeque<RadarReading>,
}

pub trait CruiseControl {
    fn tick(&self, opt_ahead: Option<&Car>) -> Car;
    fn update_position(&mut self);

    fn get_relative_target(&self) -> f32;
    fn get_absolute_target(&self) -> f32;

    fn is_car_ahead_visible(&self, opt_ahead: Option<&Car>) -> CruiseControlPositionEnum;

    fn calc_acceleration_percentage(&self, ahead: &Car) -> f32;
    fn calc_braking_percentage(&self, ahead: &Car) -> f32;
    fn new_acceleration_delta(&self) -> f32;

    fn read_radar(&mut self, opt_ahead: Option<&Car>);
    fn average_speed_front(&self) -> Vec<Option<f32>>;
}

impl Car {
    pub fn position_m(&self) -> f32 {
        self.position_m
    }

    pub fn current_speed_ms(&self) -> f32 {
        self.current_speed_ms
    }

    pub fn new(config: CarConfig) -> Self {
        Car {
            position_m: config.position_m,
            current_speed_ms: config.current_speed_ms,
            cruise_speed_ms: config.cruise_speed_ms,
            min_gap_m: config.min_gap_m,
            time_headway_sec: config.time_headway_sec,
            acceleration_mssq: config.acceleration_mssq,
            braking_mssq: config.braking_mssq,
            radar_readings: VecDeque::new(),
        }
    }
}

impl CruiseControl for Car {
    fn tick(&self, opt_ahead: Option<&Car>) -> Car {
        let mut out = self.clone();
        out.read_radar(opt_ahead);
        out.update_position();
        out
    }

    fn update_position(&mut self) {
        self.position_m += self.current_speed_ms * TICK_SECONDS;
    }

    fn get_relative_target(&self) -> f32 {
        return self.current_speed_ms * self.time_headway_sec;
    }

    fn get_absolute_target(&self) -> f32 {
        return self.get_relative_target() + self.position_m;
    }

    fn is_car_ahead_visible(&self, opt_ahead: Option<&Car>) -> CruiseControlPositionEnum {
        match opt_ahead {
            Some(ahead) => {
                if ahead.position_m > self.position_m + 200.0 {
                    return CruiseControlPositionEnum::NotVisible;
                }

                if ahead.position_m > self.get_absolute_target() {
                    return CruiseControlPositionEnum::VisibleAheadTarget;
                } else if ahead.position_m == self.get_absolute_target() {
                    return CruiseControlPositionEnum::VisibleOnTarget;
                } else {
                    return CruiseControlPositionEnum::VisibleBehindTarget;
                }
            }
            None => return CruiseControlPositionEnum::NotVisible,
        }
    }

    fn calc_acceleration_percentage(&self, ahead: &Car) -> f32 {
        let start = self.get_absolute_target();
        let end = self.position_m + 200.0;
        let length = end - start;

        (ahead.position_m - start) / length
    }

    fn calc_braking_percentage(&self, ahead: &Car) -> f32 {
        /*

        |----|-----x----|--------|
        |    |     |    |        ^-- end of radar
        |    |     |    ^----------- target point
        |    |     ^---------------- car ahead
        |    ^---------------------- self.min_gap
        ^--------------------------- 0.0

        I need to find how far ahead x is between self.min_gap and target point.
        Subtract self.min_gap from everything

        */

        (ahead.position_m - self.position_m - self.min_gap_m)
            / (self.get_relative_target() - self.min_gap_m)
            * -1.0
    }

    fn new_acceleration_delta(&self) -> f32 {
        match self.radar_readings.get(0) {
            Some(last_reading) => {
                match last_reading.relative_speed {
                    Some(relative_speed) => {
                        (self.current_speed_ms + relative_speed).clamp(self.braking_mssq, self.acceleration_mssq)
                    }
                    None => 0.0
                }
            }
            None => self.acceleration_mssq
        }
    }

    fn read_radar(&mut self, opt_ahead: Option<&Car>) {
        let radar_distance_m = match opt_ahead {
            Some(ahead) => {
                let ahead_rel_pos = ahead.position_m - self.position_m;
                if ahead_rel_pos > 200.0 {
                    None
                } else {
                    Some(ahead_rel_pos)
                }
            }
            None => None,
        };

        // radar_readings should be capped at 10 entries.
        // TODO: make "9" a constant
        if self.radar_readings.len() > 9 {
            self.radar_readings.pop_back();
        }

        // We have position in front, read previous position and figure out approx speed
        match (radar_distance_m, self.radar_readings.get(0).copied()) {
            (Some(distance_m), Some(last_reading)) => {
                let relative_speed = (distance_m - last_reading.distance_m) / TICK_SECONDS;
                let distance_from_target = distance_m - self.get_relative_target();
                let distance_from_target_s =
                    distance_from_target / relative_speed / TICK_SECONDS * -1.0;

                dbg!(
                    relative_speed,
                    distance_from_target,
                    distance_from_target_s,
                    self.get_relative_target()
                );

                self.radar_readings.push_front(RadarReading {
                    distance_m,
                    relative_speed: Some(relative_speed),
                    distance_from_target_s: Some(distance_from_target_s),
                });
            }
            (Some(distance_m), None) => {
                self.radar_readings.push_front(RadarReading {
                    distance_m,
                    relative_speed: None,
                    distance_from_target_s: None,
                });
            }
            _ => self.radar_readings.clear(),
        }
    }

    fn average_speed_front(&self) -> Vec<Option<f32>> {
        let mut speeds: Vec<Option<f32>> = Vec::new();
        for (i, x) in self.radar_readings.iter().enumerate() {
            match self.radar_readings.get(i + 1) {
                Some(prev) => speeds.push(Some((x.distance_m - prev.distance_m) / TICK_SECONDS)),
                None => {}
            }
        }

        speeds
    }
}
