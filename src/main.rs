mod structs;

use structs::{Car, CarConfig, CruiseControl};

const TICK_SECONDS: f32 = 0.1;

fn setup() -> Vec<Car> {
    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 10.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 3.0,
        braking_mssq: 5.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 150.0,
        current_speed_ms: 12.0,
        cruise_speed_ms: 12.0,
        min_gap_m: 10.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 3.0,
        braking_mssq: 5.0,
    });

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
            let newcar = x.tick(v.get(i + 1));
            z.push(newcar);
        }

        println!(
            "{}: {}: {}, {}",
            z[0].position_m(),
            z[1].position_m(),
            z[0].current_speed_ms(),
            z[1].current_speed_ms()
        );

        v = z;
    }
}
