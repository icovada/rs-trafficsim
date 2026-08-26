#[cfg(test)]
mod tests;

const TICK_SECONDS: f32 = 0.1;

pub struct Car {
    position_m: f32,
    current_speed_ms: f32, //meters/second
    cruise_speed_ms: f32,  //meters/second
    min_gap_m: f32,
    time_headway_sec: f32,
    acceleration_mssq: f32, // meters/second^2
    braking_mssq: f32,      // meters/second^2
}

#[derive(PartialEq, Eq, Debug)]
pub enum CruiseControlPositionEnum {
    VisibleAheadTarget,
    VisibleBehindTarget,
    VisibleOnTarget,
    NotVisible,
}

pub trait CruiseControl {
    fn update_position(&mut self, tick_seconds: f32);

    fn get_relative_target(&self) -> f32;
    fn get_absolute_target(&self) -> f32;

    fn is_car_ahead_visible(&self, opt_ahead: Option<&Car>) -> CruiseControlPositionEnum;

    fn calc_acceleration_percentage(&self, opt_ahead: Option<&Car>) -> f32;
    fn calc_braking_percentage(&self, opt_ahead: Option<&Car>) -> f32;
    fn new_acceleration_delta(&self, opt_ahead: Option<&Car>) -> f32;
}

impl CruiseControl for Car {
    fn update_position(&mut self, tick_seconds: f32) {
        self.position_m += self.current_speed_ms * tick_seconds;
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

                if ahead.position_m > self.get_relative_target() {
                    return CruiseControlPositionEnum::VisibleAheadTarget;
                } else if ahead.position_m == self.get_relative_target() {
                    return CruiseControlPositionEnum::VisibleOnTarget;
                } else {
                    return CruiseControlPositionEnum::VisibleBehindTarget;
                }
            }
            None => return CruiseControlPositionEnum::NotVisible,
        }
    }

    fn calc_acceleration_percentage(&self, opt_ahead: Option<&Car>) -> f32 {
        match opt_ahead {
            Some(ahead) => {
                let start = self.get_relative_target();
                let end = 200.0;
                let length = end - start;

                println!("{} {} {}", start, end, length);

                (ahead.position_m - start) / length
            }
            None => 1.0,
        }
    }

    fn calc_braking_percentage(&self, opt_ahead: Option<&Car>) -> f32 {
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

        match opt_ahead {
            Some(ahead) => {
                (ahead.position_m - self.position_m - self.min_gap_m)
                    / (self.get_relative_target() - self.min_gap_m)
                    * -1.0
            }
            None => 0.0,
        }
    }

    fn new_acceleration_delta(&self, opt_ahead: Option<&Car>) -> f32 {
        match self.is_car_ahead_visible(opt_ahead) {
            CruiseControlPositionEnum::NotVisible => 100.0,
            CruiseControlPositionEnum::VisibleOnTarget => 0.0,
            CruiseControlPositionEnum::VisibleAheadTarget => {
                self.calc_acceleration_percentage(opt_ahead) * self.acceleration_mssq
            }
            CruiseControlPositionEnum::VisibleBehindTarget => {
                self.calc_braking_percentage(opt_ahead) * self.braking_mssq
            }
        }
    }
}

fn setup() -> Vec<Car> {
    let car1 = Car {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 10.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 3.0,
        braking_mssq: 5.0,
    };

    let car2 = Car {
        position_m: 100.0,
        current_speed_ms: 12.0,
        cruise_speed_ms: 12.0,
        min_gap_m: 10.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 3.0,
        braking_mssq: 5.0,
    };

    let v = vec![car1, car2];

    v
}

fn cruise_control_variation(behind: &Car, opt_ahead: Option<&Car>) -> f32 {
    match opt_ahead {
        Some(ahead) => {
            // Find whether the car ahead is inside the radar zone
            if ahead.position_m > behind.position_m + 200.0 {
                return 0.0;
            }

            let target_cruise_control_position =
                behind.position_m + behind.current_speed_ms * behind.time_headway_sec;
            let rel_target_pos = target_cruise_control_position - behind.position_m;
            let distance_between_cars_m = ahead.position_m - behind.position_m;

            if ahead.position_m > target_cruise_control_position {
                let radar_available_ahead_m = 200.0 - rel_target_pos;
                let perc_ahead = distance_between_cars_m / (radar_available_ahead_m);

                println!("ahead {}", perc_ahead);

                let acceleration = behind.acceleration_mssq * perc_ahead;
                return acceleration.clamp(0.0, behind.acceleration_mssq) * TICK_SECONDS;
            } else if ahead.position_m < target_cruise_control_position {
                let perc_behind = (distance_between_cars_m - behind.min_gap_m)
                    / (rel_target_pos - behind.min_gap_m);
                println!("behind {}", perc_behind);
                let braking = behind.braking_mssq * perc_behind;
                return braking.clamp(behind.braking_mssq, 0.0) * TICK_SECONDS;
            } else {
                return 0.0;
            }
        }
        None => return 0.0,
    }
}

fn tick(v: &Vec<Car>) -> Vec<Car> {
    let mut z: Vec<Car> = Vec::new();

    for (i, x) in v.iter().enumerate() {
        let new_speed = x.current_speed_ms + cruise_control_variation(x, v.get(i + 1));
        let car = Car {
            position_m: x.position_m + (new_speed.clamp(0.0, x.cruise_speed_ms) * TICK_SECONDS),
            current_speed_ms: new_speed.clamp(0.0, x.cruise_speed_ms),
            cruise_speed_ms: x.cruise_speed_ms,
            min_gap_m: x.min_gap_m,
            time_headway_sec: x.time_headway_sec,
            acceleration_mssq: x.acceleration_mssq,
            braking_mssq: x.braking_mssq,
        };

        z.push(car);
    }

    println!(
        "{}: {}, {}",
        z[1].position_m - z[0].position_m,
        z[0].current_speed_ms,
        z[1].current_speed_ms
    );
    z
}

fn main() {
    let mut v = setup();

    let mut maxtick = 250;
    while maxtick > 0 {
        maxtick -= 1;

        let mut z = tick(&v);
        v = Vec::new();
        v.append(&mut z);
    }
}
