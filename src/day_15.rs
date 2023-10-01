//! Day 15: Beacon Exclusion Zone
//!
//! https://adventofcode.com/2022/day/15

use crate::lib::vector_2d::Vector2D;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> i64 {
    let sensors = parser::parse(input);
    let y = 2_000_000;

    count_positions_where_a_beacon_cannot_be_present(sensors, y)
}

pub fn part_2(input: &str) -> i64 {
    let sensors = parser::parse(input);
    let limit = 4000000;
    let beacon = find_distress_beacon(sensors, limit);
    tuning_frequency(beacon)
}

fn find_distress_beacon(sensors: Vec<Sensor>, limit: i64) -> Vector2D<i64> {
    'rows: for row in 0..=limit {
        let mut ranges: Vec<RangeInclusive<i64>> = sensors.iter().filter_map(|s| s.x_range_within_radius_given_y(row)).collect();
        ranges.sort_by_key(|r| *r.start());

        let mut col = 0;
        for range in ranges {
            if col > limit {
                continue 'rows;
            }
            if *range.start() > col {
                return Vector2D::from((col, row));
            }
            col = max(col, *range.end() + 1);
        }
    }

    panic!("beacon not found");
}

fn tuning_frequency(beacon: Vector2D<i64>) -> i64 {
    beacon.x * 4000000 + beacon.y
}

fn count_positions_where_a_beacon_cannot_be_present(sensors: Vec<Sensor>, y: i64) -> i64 {
    let mut ranges: Vec<RangeInclusive<i64>> = sensors.iter().filter_map(|s| s.x_range_within_radius_given_y(y)).collect();
    ranges.sort_by_key(|r| *r.start());

    let mut i = i64::MIN;
    let mut count = 0;
    for range in ranges {
        if *range.end() > i {
            if *range.start() > i {
                count += 1;
                i = *range.start();
            }
            count += range.end() - i;
            i = *range.end();
        }
    }

    let beacons: HashSet<Vector2D<i64>> = sensors.iter().map(|s| s.closest_beacon).collect();
    for beacon in beacons {
        if beacon.y == y {
            count -= 1;
        }
    }

    count
}

struct Sensor {
    at: Vector2D<i64>,
    closest_beacon: Vector2D<i64>,
}

impl Sensor {
    /// Return the x-range of points that have a manhattan distance to the sensor that is
    /// less or equal to the manhattan distance from the sensor to its nearest beacon.
    ///
    /// Return None if there are no such points.
    fn x_range_within_radius_given_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let manhattan_dist_from_sensor_to_beacon = self.at.manhattan_distance(self.closest_beacon);
        let distance_from_sensor_to_y_plane = (self.at.y - y).abs();

        if distance_from_sensor_to_y_plane <= manhattan_dist_from_sensor_to_beacon {
            let x_range_to_either_side = manhattan_dist_from_sensor_to_beacon - distance_from_sensor_to_y_plane;
            Some((self.at.x - x_range_to_either_side)..=(self.at.x + x_range_to_either_side))
        } else {
            None
        }
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Sensor> {
        all_consuming(many0(parse_sensor_line))(s).unwrap().1
    }

    fn parse_sensor_line(s: &str) -> IResult<&str, Sensor> {
        let (s, _) = tag("Sensor at x=")(s)?;
        let (s, sensor_x) = i64(s)?;
        let (s, _) = tag(", y=")(s)?;
        let (s, sensor_y) = i64(s)?;
        let (s, _) = tag(": closest beacon is at x=")(s)?;
        let (s, beacon_x) = i64(s)?;
        let (s, _) = tag(", y=")(s)?;
        let (s, beacon_y) = i64(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            Sensor {
                at: Vector2D::from((sensor_x, sensor_y)),
                closest_beacon: Vector2D::from((beacon_x, beacon_y)),
            },
        ))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

#[test]
fn part_1_example() {
    let sensors = parser::parse(EXAMPLE);
    let y = 10;
    let c = count_positions_where_a_beacon_cannot_be_present(sensors, y);
    assert_eq!(c, 26);
}

#[test]
fn part_2_example() {
    let sensors = parser::parse(EXAMPLE);
    let limit = 20;
    let beacon = find_distress_beacon(sensors, limit);
    let tuning_freq = tuning_frequency(beacon);
    assert_eq!(tuning_freq, 56000011);
}
