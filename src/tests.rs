
use crate::*;

#[test]
fn test_update_position() {

    let mut car1 = Car{
        position_m: 0.0,
        current_speed_ms: 30.0,
        cruise_speed_ms: 30.0,
        min_gap_m: 3.0,
        time_headway_sec: 3.0,
        acceleration_mssq: 5.0,
        braking_mssq: 10.0,
    };

    car1.update_position(0.1);

    assert_eq!(car1.position_m, 3.0)


}