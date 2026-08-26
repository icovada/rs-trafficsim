use crate::*;

#[test]
fn test_update_position() {
    let mut car1 = Car {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    };

    car1.update_position(TICK_SECONDS);

    assert_eq!(car1.position_m, 3.0)
}

#[test]
fn test_get_relative_target() {
    let car1 = Car {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    };

    assert_eq!(car1.get_relative_target(), 90.0);
}

#[test]
fn test_is_car_ahead_visible() {
    let car1 = Car {
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    };

    let opt_none: Option<&Car> = None;
    assert_eq!(car1.is_car_ahead_visible(opt_none), CruiseControlPositionEnum::NotVisible);

    let mut car2 = Car {
        position_m: 250.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    };

    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.is_car_ahead_visible(opt_some), CruiseControlPositionEnum::NotVisible);

    car2.position_m = 190.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.is_car_ahead_visible(opt_some), CruiseControlPositionEnum::VisibleAheadTarget);

    car2.position_m = 90.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.is_car_ahead_visible(opt_some), CruiseControlPositionEnum::VisibleOnTarget);

    car2.position_m = 30.0;
    let opt_some: Option<&Car> = Some(&mut car2);
    assert_eq!(car1.is_car_ahead_visible(opt_some), CruiseControlPositionEnum::VisibleBehindTarget);


}

