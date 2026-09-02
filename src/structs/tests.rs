use super::*;
use std::collections::VecDeque;

#[test]
fn test_update_position() {
    let mut car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    car1.update_position();

    assert_eq!(car1.position_m, 3.0)
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
        braking_mssq: 10.0,
    });

    assert_eq!(car1.get_relative_target(), 90.0);
}

#[test]
fn test_is_car_ahead_visible() {
    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let opt_none: Option<&Car> = None;
    assert_eq!(
        car1.is_car_ahead_visible(opt_none),
        CruiseControlPositionEnum::NotVisible
    );

    let mut car2 = Car::new(CarConfig {
        position_m: 250.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(
        car1.is_car_ahead_visible(opt_some),
        CruiseControlPositionEnum::NotVisible
    );

    car2.position_m = 190.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(
        car1.is_car_ahead_visible(opt_some),
        CruiseControlPositionEnum::VisibleAheadTarget
    );

    car2.position_m = 90.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(
        car1.is_car_ahead_visible(opt_some),
        CruiseControlPositionEnum::VisibleOnTarget
    );

    car2.position_m = 30.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(
        car1.is_car_ahead_visible(opt_some),
        CruiseControlPositionEnum::VisibleBehindTarget
    );
}

#[test]
fn test_calc_braking_percentage() {
    /*

    |----|-----x----|--------|
    |    |     |    |        ^-- end of radar : 200
    |    |     |    ^----------- target point : 90.0
    |    |     ^---------------- car ahead    : 45.0
    |    ^---------------------- self.min_gap : 30.0
    ^--------------------------- 0.0

    I need to find how far ahead x is between self.min_gap and target point.


    */

    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 60.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    assert_eq!(car1.get_absolute_target(), 90.0);

    // min 30.0
    // max 30.0
    // halfpoint 60.0 -> brake 50% -> -0.5

    assert_eq!(car1.calc_braking_percentage(&car2), -0.5);
}

#[test]
fn test_calc_acceleration_percentage() {
    /*

    |----|-----|----x--------|
    |    |     |    |        ^-- end of radar : 200
    |    |     |    ^----------- car ahead    : 145.0
    |    |     ^---------------- target point : 90.0
    |    ^---------------------- self.min_gap : 30.0
    ^--------------------------- 0.0

    I need to find how far ahead x is between target point e 200.0.


    */

    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 145.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    assert_eq!(car1.get_absolute_target(), 90.0);

    // min 90.0
    // max 200.0
    // halfpoint 145 -> accel 50% -> 0.5

    assert_eq!(car1.calc_acceleration_percentage(&car2), 0.5);
}

#[test]
fn test_new_acceleration_delta() {
    /*

    |----|-----|----x--------|
    |    |     |    |        ^-- end of radar : 200
    |    |     |    ^----------- car ahead    : 145.0
    |    |     ^---------------- target point : 90.0
    |    ^---------------------- self.min_gap : 30.0
    ^--------------------------- 0.0

    I need to find how far ahead x is between target point e 200.0.


    */

    let car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let mut car2 = Car::new(CarConfig {
        position_m: 145.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.new_acceleration_delta(opt_some), 2.5);

    car2.position_m = 60.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.new_acceleration_delta(opt_some), -5.0);
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
        braking_mssq: 10.0,
    });

    let car2 = Car::new(CarConfig {
        position_m: 60.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let opt_car: Option<&Car> = Some(&car2);
    let out = car1.tick(opt_car);

    dbg!(out.new_acceleration_delta(opt_car));

    dbg!(out.position_m, out.current_speed_ms);

    assert_eq!(out.position_m, 2.95)
}

#[test]
fn test_read_radar() {
    //TODO: this test currently does nothing
    let mut car1 = Car::new(CarConfig {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 30.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    let mut car2 = Car::new(CarConfig {
        position_m: 60.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    });

    // run 20 readings, check we only get 10 in array
    for _ in 0..20 {
        car1.read_radar(Some(&car2));
        car1.position_m = car1.position_m + 1.0;
    }

    assert_eq!(car1.radar_readings.len(), 10);
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        Some(RadarReading {
            distance_m: 41.0,
            relative_speed: Some(-10.0),
            distance_from_target_s: Some(-49.0)
        })
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
        Some(RadarReading {
            distance_m: 50.1,
            relative_speed: Some(0.99998474),
            distance_from_target_s: Some(399.0061)
        })
    );

    // Move car2 way off. Expect None as reading
    car2.position_m = 350.0;
    car1.read_radar(Some(&car2));
    assert_eq!(*car1.radar_readings.get(0).unwrap(), None);

    // Move car2 closer. Expect Some reading with None relative speed (need 2 measurements)
    car2.position_m = 150.0;
    car1.read_radar(Some(&car2));
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        Some(RadarReading {
            distance_m: 140.0,
            relative_speed: None,
            distance_from_target_s: None
        })
    );

    // Move car2 close. Expect full reading.
    car2.position_m = 100.0;
    car1.read_radar(Some(&car2));
    assert_eq!(
        *car1.radar_readings.get(0).unwrap(),
        Some(RadarReading {
            distance_m: 90.0,
            relative_speed: Some(-500.0),
            distance_from_target_s: Some(0.0)
        })
    );
}
