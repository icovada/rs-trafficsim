#[cfg(test)]
mod tests;

const TICK_SECONDS: f32 = 0.1;

#[derive(Clone, Copy)]
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
    fn tick(&self, opt_ahead: Option<&Car>) -> Car;
    fn update_position(&mut self);

    fn get_relative_target(&self) -> f32;
    fn get_absolute_target(&self) -> f32;

    fn is_car_ahead_visible(&self, opt_ahead: Option<&Car>) -> CruiseControlPositionEnum;

    fn calc_acceleration_percentage(&self, ahead: &Car) -> f32;
    fn calc_braking_percentage(&self, ahead: &Car) -> f32;
    fn new_acceleration_delta(&self, opt_ahead: Option<&Car>) -> f32;
}

impl CruiseControl for Car {
    fn tick(&self, opt_ahead: Option<&Car>) -> Car{
        let accel_delta: f32 = self.new_acceleration_delta(opt_ahead) * TICK_SECONDS;
        
        let mut out = self.clone();
        out.current_speed_ms = (out.current_speed_ms + accel_delta).clamp(0.0, out.cruise_speed_ms);
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

    fn calc_acceleration_percentage(&self, ahead: &Car) -> f32 {
        let start = self.get_relative_target();
        let end = 200.0;
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

    fn new_acceleration_delta(&self, opt_ahead: Option<&Car>) -> f32 {
        match self.is_car_ahead_visible(opt_ahead) {
            CruiseControlPositionEnum::NotVisible => 100.0,
            CruiseControlPositionEnum::VisibleOnTarget => 0.0,
            CruiseControlPositionEnum::VisibleAheadTarget => {
                self.calc_acceleration_percentage(opt_ahead.unwrap()) * self.acceleration_mssq
            }
            CruiseControlPositionEnum::VisibleBehindTarget => {
                self.calc_braking_percentage(opt_ahead.unwrap()) * self.braking_mssq
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
        position_m: 150.0,
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

fn main() {
    let mut v = setup();

    let mut maxtick = 100;
    while maxtick > 0 {
        maxtick -= 1;

        let mut z: Vec<Car> = Vec::new();

        for (i, x) in v.iter().enumerate() {
            let newcar = x.tick(v.get(i+1));
            z.push(newcar);
        }

        println!("{}: {}: {}, {}", z[0].position_m, z[1].position_m, z[0].current_speed_ms, z[1].current_speed_ms);

        v = z;
    }
}
