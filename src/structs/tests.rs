use super::*;

#[test]
fn test_update_position() {
    let mut car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    car1.update_position();

    assert_eq!(car1.position_m, 3.0);
    assert_eq!(car1.cruise_speed_ms, 30.0);
}

#[test]
fn test_get_relative_target() {
    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    assert_eq!(car1.get_relative_target(), 90.0);
}

#[test]
fn test_car_tick() {
    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 60.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    let opt_car: Option<&Car> = Some(&car2);
    let out = car1.tick(opt_car);

    assert_eq!(out.position_m, 3.0)
}

#[test]
fn test_read_radar() {
    let mut car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    let mut car2 = Car::new(CarConfig {
        position_m: 60.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    // run 20 readings, check we only get 10 in array
    for _ in 0..20 {
        car1.read_radar(Some(&car2));
        car1.position_m = car1.position_m + 1.0;
    }

    assert_eq!(car1.radar_readings.len(), 10);
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        RadarReading {
            distance_m: 41.0,
            relative_speed: Some(-10.0),
            distance_from_target_s: Some(-4.90)
        }
    );

    // Reset car position, read twice
    car1.position_m = 0.0;
    car2.position_m = 50.0;
    car1.read_radar(Some(&car2));

    car1.position_m = 10.0;
    car2.position_m = 60.1;
    car1.read_radar(Some(&car2));

    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        RadarReading {
            distance_m: 50.1,
            relative_speed: Some(0.99998474),
            distance_from_target_s: Some(39.90061)
        }
    );

    // Move car2 way off. Vectory empty
    car2.position_m = 350.0;
    car1.read_radar(Some(&car2));
    assert_eq!(car1.radar_readings.len(), 0);

    // Move car2 closer. Expect Some reading with None relative speed (need 2 measurements)
    car2.position_m = 150.0;
    car1.read_radar(Some(&car2));
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        RadarReading {
            distance_m: 140.0,
            relative_speed: None,
            distance_from_target_s: None
        }
    );

    // Move car2 close. Expect full reading.
    car2.position_m = 100.0;
    car1.read_radar(Some(&car2));
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        RadarReading {
            distance_m: 90.0,
            relative_speed: Some(-500.0),
            distance_from_target_s: Some(0.0)
        }
    );
}

#[test]
fn test_emergency_brake() {
    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 5.0,
        current_speed_ms: 20.0,
        cruise_speed_ms: 20.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: -10.0,
    });

    let car1 = car1.tick(Some(&car2));
    let car2 = car2.tick(None);

    let car1 = car1.tick(Some(&car2));
    let car2 = car2.tick(None);

    assert!(car1.current_speed_ms < 30.0);

}
