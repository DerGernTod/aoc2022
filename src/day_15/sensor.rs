use std::{str::FromStr, num::ParseIntError};

use super::coords::Coords;

pub struct Sensor {
    location: Coords,
    range: i32
}

impl Sensor {
    pub fn new(location: Coords, closest_beacon: Coords) -> Sensor {
        let range = (location.0.abs_diff(closest_beacon.0)
            + location.1.abs_diff(closest_beacon.1)) as i32;
        Sensor { location, range }
    }
    pub fn calc_covered_row_start_end(&self, row: i32) -> (i32, i32) {
        let dist = self.location.1.abs_diff(row) as i32;
        let h_range = (self.range - dist).max(0);
        (self.location.0 - h_range, self.location.0 + h_range)
    }
    pub fn calc_covered_col_start_end(&self, col: i32) -> (i32, i32) {
        let dist = self.location.0.abs_diff(col) as i32;
        let v_range = (self.range - dist).max(0);
        (self.location.1 - v_range, self.location.1 + v_range)
    }
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl = s
            .replace("Sensor at x=", "")
            .replace(", y=", ";")
            .replace(": closest beacon is at x=", ";")
            .replace(", y=", ";");
        let mut spl = spl.split(';')
            .map(|num| num.parse::<i32>());
        let location = Coords(spl.next().unwrap()?, spl.next().unwrap()?);
        let closest_beacon = Coords(spl.next().unwrap()?, spl.next().unwrap()?);
        Ok(Sensor::new(location, closest_beacon))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_15::coords::Coords;

    use super::Sensor;

    #[test]
    fn test_covered_row() {
        let sensor = Sensor::new(Coords(8, 7), Coords(2, 10));
        let covered = sensor.calc_covered_row_start_end(10);
        assert_eq!(covered, (2, 14));
    }
}