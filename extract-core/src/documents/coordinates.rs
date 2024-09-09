use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    SCREEN,
    CARTESIAN,
}

impl Orientation {
    fn values(&self) -> (i8, i8) {
        match self {
            Orientation::SCREEN => (1, -1), // Origin in top left, y increases in the down direction
            Orientation::CARTESIAN => (1, 1), // Origin in bottom left, y increases in upward direction
        }
    }
}

/// Performs a linear transformation of a coordinate from one system to another.
/// old_t: The coordinate value in the original system.
/// old_t_max: The maximum value of the coordinate in the original system.
/// new_t_max: The maximum value of the coordinate in the new system.
/// t_orientation: The orientation factor, which can be either 1 or -1 depending on the direction
/// of the axis (increasing or decreasing).
fn convert_coordinate(old_t: f64, old_t_max: f64, new_t_max: f64, t_orientation: i8) -> f64 {
    // old_t / old_t_max: This normalizes the coordinate to a range of [0, 1].
    // Adjusting coordinates using (1 - t_orientation) / 2.0
    //  * When t_orientation is 1 (e.g., Cartesian coordinates), this part becomes 0, so it doesn't affect the result.
    //  * When t_orientation is -1 (e.g., screen coordinates), this part becomes 1, so it flips the coordinate.

    // In summary when t_orientation = 1, This simplifies to old_t / old_t_max,
    //  and when t_orientation = -1, this simplifies to 1.0 - old_t / old_t_max
    ((1.0 - (old_t / old_t_max)) * ((1 - t_orientation) / 2) as f64
        + (old_t / old_t_max) * ((1 + t_orientation) / 2) as f64)
        * new_t_max
}

/// A finite coordinate plane with given width and height.
#[derive(Debug, PartialEq)]
pub struct CoordinateSystem {
    width: f64,
    height: f64,
    orientation: Orientation,
}

impl CoordinateSystem {
    pub fn new(width: f64, height: f64, orientation: Orientation) -> Self {
        Self {
            width,
            height,
            orientation,
        }
    }

    /// Convert to this coordinate system from a relative coordinate system.
    pub fn convert_from_relative(&self, x: f64, y: f64) -> (f64, f64) {
        let (x_orientation, y_orientation) = self.orientation.values();
        let new_x = convert_coordinate(x, 1.0, self.width, x_orientation);
        let new_y = convert_coordinate(y, 1.0, self.height, y_orientation);
        (new_x, new_y)
    }

    // Convert from this coordinate system to a relative coordinate system.
    // Relative is x and y in the range [0, 1]. X-axis and Y-axis are both upwards.
    pub fn convert_to_relative(&self, x: f64, y: f64) -> (f64, f64) {
        let (x_orientation, y_orientation) = self.orientation.values();
        let new_x = convert_coordinate(x, self.width, 1.0, x_orientation);
        let new_y = convert_coordinate(y, self.height, 1.0, y_orientation);
        (new_x, new_y)
    }

    /// Convert from this coordinate system to another given coordinate system.
    pub fn convert_coordinates_to_new_system(
        &self,
        new_system: &CoordinateSystem,
        x: f64,
        y: f64,
    ) -> (f64, f64) {
        let (rel_x, rel_y) = self.convert_to_relative(x, y);
        new_system.convert_from_relative(rel_x, rel_y)
    }

    /// Convert (x, y) coordinates from current system to another coordinate system.
    pub fn _convert_multiple_coordinates_to_new_system(
        &self,
        new_system: &CoordinateSystem,
        coordinates: &[(f64, f64)],
    ) -> Vec<(f64, f64)> {
        coordinates
            .iter()
            .map(|&(x, y)| self.convert_coordinates_to_new_system(new_system, x, y))
            .collect()
    }
}

impl fmt::Display for CoordinateSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CoordinateSystem(width: {}, height: {}, orientation: {:?})",
            self.width, self.height, self.orientation
        )
    }
}

pub struct RelativeCoordinateSystem;

impl RelativeCoordinateSystem {
    pub fn new() -> CoordinateSystem {
        CoordinateSystem::new(1.0, 1.0, Orientation::CARTESIAN)
    }
}

pub struct PixelSpace;

impl PixelSpace {
    pub fn new(width: f64, height: f64) -> CoordinateSystem {
        CoordinateSystem::new(width, height, Orientation::SCREEN)
    }
}

pub struct PointSpace;

impl PointSpace {
    pub fn new(width: f64, height: f64) -> CoordinateSystem {
        CoordinateSystem::new(width, height, Orientation::CARTESIAN)
    }
}

#[cfg(test)]
mod tests {
    use crate::documents::coordinates::{
        convert_coordinate, CoordinateSystem, Orientation, RelativeCoordinateSystem,
    };
    #[test]
    fn test_convert_to_coordinate() {
        assert_eq!(0.0, convert_coordinate(0.0, 7.0, 5.0, 1));
        assert_eq!(5.0, convert_coordinate(7.0, 7.0, 5.0, 1));
        assert_eq!(5.0, convert_coordinate(0.0, 7.0, 5.0, -1));
        assert_eq!(0.0, convert_coordinate(7.0, 7.0, 5.0, -1));
    }

    #[test]
    fn test_convert_to_relative() {
        let point_space = CoordinateSystem::new(100.0, 300.0, Orientation::CARTESIAN);
        assert_eq!((0.0, 0.0), point_space.convert_to_relative(0.0, 0.0));
        assert_eq!((0.8, 0.4), point_space.convert_to_relative(80.0, 120.0));
        assert_eq!((1.0, 1.0), point_space.convert_to_relative(100.0, 300.0));

        let pixel_space = CoordinateSystem::new(100.0, 300.0, Orientation::SCREEN);
        assert_eq!((0.0, 1.0), pixel_space.convert_to_relative(0.0, 0.0));
        assert_eq!((0.8, 0.6), pixel_space.convert_to_relative(80.0, 120.0));
        assert_eq!((1.0, 0.0), pixel_space.convert_to_relative(100.0, 300.0));
    }

    #[test]
    fn test_convert_from_relative() {
        let point_space = CoordinateSystem::new(100.0, 300.0, Orientation::CARTESIAN);
        assert_eq!((80.0, 120.0), point_space.convert_from_relative(0.8, 0.4));

        let pixel_space = CoordinateSystem::new(100.0, 300.0, Orientation::SCREEN);
        assert_eq!((80.0, 120.0), pixel_space.convert_from_relative(0.8, 0.6));
    }

    #[test]
    fn test_convert_to_new_system() {
        for vals in [
            (
                Orientation::CARTESIAN,
                Orientation::CARTESIAN,
                80.0,
                120.0,
                800.0,
                1200.0,
            ),
            (
                Orientation::CARTESIAN,
                Orientation::SCREEN,
                80.0,
                120.0,
                800.0,
                800.0,
            ),
            (
                Orientation::SCREEN,
                Orientation::CARTESIAN,
                80.0,
                120.0,
                800.0,
                800.0,
            ),
            (
                Orientation::SCREEN,
                Orientation::SCREEN,
                80.0,
                120.0,
                800.0,
                1200.0,
            ),
        ] {
            let coord1 = CoordinateSystem::new(100.0, 200.0, vals.0);
            let coord2 = CoordinateSystem::new(1000.0, 2000.0, vals.1);
            assert_eq!(
                (vals.4, vals.5),
                coord1.convert_coordinates_to_new_system(&coord2, vals.2, vals.3)
            );
        }
    }

    #[test]
    fn test_convert_to_new_system_relative() {
        for vals in [
            (100.0, 300.0, Orientation::CARTESIAN, 80.0, 120.0, 0.8, 0.4),
            (100.0, 300.0, Orientation::SCREEN, 80.0, 120.0, 0.8, 0.6),
        ] {
            let coord1 = CoordinateSystem::new(vals.0, vals.1, vals.2);
            let coord2 = RelativeCoordinateSystem::new();
            assert_eq!(
                (vals.5, vals.6),
                coord1.convert_coordinates_to_new_system(&coord2, vals.3, vals.4)
            );
        }
    }
}
