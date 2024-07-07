use crate::vec3d::Vec3d;

/// A plane in 3D space
#[derive(Copy, Clone, Debug)]
pub struct Plane {
    /// The normal vector of the plane
    pub normal: Vec3d,
    /// The distance from the origin to the plane
    pub distance: f64
}

impl Plane {
    /// Create a new plane
    pub fn new(normal: &Vec3d, distance: f64) -> Plane {
        Plane {
            normal: normal.normalize(),
            distance
        }
    }

    /// Create a new plane from a normal and a point on the plane
    pub fn from_point(normal: &Vec3d, point: &Vec3d) -> Plane {
        let distance = -normal.dot(&point);
        Plane {
            normal: normal.normalize(),
            distance
        }
    }

    /// The XY plane
    pub fn xy() -> Plane {
        Plane::new(&Vec3d::k(), 0.0)
    }

    /// The XZ plane
    pub fn xz() -> Plane {
        Plane::new(&Vec3d::j(), 0.0)
    }

    /// The YZ plane
    pub fn yz() -> Plane {
        Plane::new(&Vec3d::i(), 0.0)
    }

    /// Create a plane from three points
    pub fn from_points(point1: &Vec3d, point2: &Vec3d, point3: &Vec3d) -> Plane {
        let normal = (point2 - point1).cross(&(point3 - point1));
        Plane::from_point(&normal, point1)
    }

    /// Get the unsigned distance from a point to the plane
    pub fn distance_to_point(&self, point: &Vec3d) -> f64 {
        self.normal.x * point.x + self.normal.y * point.y + self.normal.z * point.z - self.distance
    }
}
