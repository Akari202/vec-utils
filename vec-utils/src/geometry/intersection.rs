use core::f64;

use thiserror::Error;

use crate::geometry::circle::Circle;
use crate::geometry::plane::Plane;
use crate::geometry::sphere::Sphere;
use crate::vec3d::Vec3d;

/// The general error enum for the crate
#[derive(Error, Debug, PartialEq, Eq)]
pub enum IntersectionError {
    /// Geometries are too far apart to intersect at all
    /// For planar geometries it can be assumed that the geometries lie in the same plane,
    /// otherwise a `OutOfPlane` error would have been returned
    #[error("The given geometries are too far apart and do not intersect at all.")]
    TooFarApart,
    /// Geometries contain one another, the surfaces dont intersect at all and all points within
    /// one are contained in the other
    #[error(
        "One of the given geometries is contained entirely within the other. The surfaces do not intersect."
    )]
    ContainedWithin,
    /// The geometries are equal and thus the entire surface would intersect
    #[error("The given geometries are identical.")]
    EqualGeometries,
    /// The geometries are not contained within the same plane and thus will never intersect
    /// Only relevant for planar geometries
    #[error("The given geometries are out of plane from each other")]
    OutOfPlane
}

/// Calculate the intersection of two spheres
/// Returns the circle of intersection
/// if the circles touch at a single point a degenerate circle is returned
/// # Errors
/// All non planar `IntersectionError` variants are returned
pub fn sphere_sphere(sphere1: &Sphere, sphere2: &Sphere) -> Result<Circle, IntersectionError> {
    if sphere1 == sphere2 {
        return Err(IntersectionError::EqualGeometries);
    }
    let center_distance = sphere1.center.distance_to(&sphere2.center);
    let radius_sum = sphere1.radius + sphere2.radius;
    if center_distance > radius_sum {
        return Err(IntersectionError::TooFarApart);
    }
    let radius_diff = (sphere1.radius - sphere2.radius).abs();
    if center_distance < radius_diff {
        return Err(IntersectionError::ContainedWithin);
    }
    // let circle_radius = (sphere1.radius.powi(2) - sphere2.radius.powi(2) + center_distance.powi(2)) / (2.0 * center_distance);
    // let circle_center = sphere1.center + (sphere2.center - sphere1.center) * (circle_radius / center_distance);

    let h =
        0.5 + (sphere1.radius.powi(2) - sphere2.radius.powi(2)) / (2.0 * center_distance.powi(2));
    let radius_of_intersection =
        (sphere1.radius.powi(2) - h.powi(2) * center_distance.powi(2)).sqrt();
    let center_of_intersection = sphere1.center + h * (sphere2.center - sphere1.center);
    let circle_normal = (sphere2.center - sphere1.center).normalize();
    Ok(Circle::new(
        &center_of_intersection,
        radius_of_intersection,
        &circle_normal
    ))
}

/// Calculate the intersection of a sphere and a plane
/// Returns the circle of intersection
/// # Errors
/// `IntersectionError::OutOfPlane` is the only possible error
pub fn sphere_plane(sphere: &Sphere, plane: &Plane) -> Result<Circle, IntersectionError> {
    let distance = plane.distance_to_point(&sphere.center);
    if distance < f64::EPSILON {
        return Ok(Circle::new(&sphere.center, sphere.radius, &plane.normal));
    }
    if distance > sphere.radius {
        return Err(IntersectionError::OutOfPlane);
    }
    if (distance - sphere.radius).abs() < f64::EPSILON {
        let circle_center = sphere.center - plane.normal * distance;
        return Ok(Circle::new(&circle_center, 0.0, &plane.normal));
    }
    let circle_radius = (sphere.radius.powi(2) - distance.powi(2)).sqrt();
    // WARN: idk why this needs to be the way it is
    let circle_center = sphere.center - plane.normal * distance;
    // dbg!(
    //     sphere,
    //     circle_radius,
    //     circle_center,
    //     plane.normal * distance,
    //     plane,
    //     distance
    // );
    Ok(Circle::new(&circle_center, circle_radius, &plane.normal))
}

/// Calculate the intersection of two circles
/// DOES NOT CALCULATE INTERSECTION BETWEEN OUT OF PLANE CIRCLES
/// Returns one, or two points of intersection
/// The points are returned as a tuple of two Vec3d
/// If the circles intersect at one point, the same point is returned twice
/// If the circles intersect at two points, the two points are returned
/// # Errors
/// All `IntersectionError` variants are possible
pub fn circle_circle(
    circle1: &Circle,
    circle2: &Circle
) -> Result<(Vec3d, Vec3d), IntersectionError> {
    // dbg!(circle1);
    // dbg!(circle2);
    if circle1 == circle2 {
        return Err(IntersectionError::EqualGeometries);
    }
    if !circle1.in_same_plane(circle2) {
        return Err(IntersectionError::OutOfPlane);
    }
    let center_distance = circle1.center.distance_to(&circle2.center);
    let radius_sum = circle1.radius + circle2.radius;
    if center_distance > radius_sum {
        return Err(IntersectionError::TooFarApart);
    }
    let radius_diff = (circle1.radius - circle2.radius).abs();
    if center_distance < radius_diff {
        return Err(IntersectionError::ContainedWithin);
    }
    let h =
        0.5 + (circle1.radius.powi(2) - circle2.radius.powi(2)) / (2.0 * center_distance.powi(2));
    let radius_of_intersection =
        (circle1.radius.powi(2) - h.powi(2) * center_distance.powi(2)).sqrt();
    let t = (circle2.center - circle1.center)
        .cross(&circle2.normal)
        .normalize();
    let center_of_intersection = circle1.center + h * (circle2.center - circle1.center);
    let point1 = center_of_intersection + t * radius_of_intersection;
    let point2 = center_of_intersection - t * radius_of_intersection;
    Ok((point1, point2))
}

/// Calculate the intersection of a sphere and a circle
/// if there is one point of intersection it is returned twice
/// # Errors
/// `IntersectionError` is returned as appropriate
pub fn sphere_circle(
    sphere: &Sphere,
    circle: &Circle
) -> Result<(Vec3d, Vec3d), IntersectionError> {
    let circle_plane = circle.get_plane();
    let sphere_circle = sphere_plane(sphere, &circle_plane)?;
    if sphere_circle.is_degenerate() {
        match circle_point_unchecked(circle, &sphere_circle.center, false) {
            Ok(_) => Ok((sphere_circle.center, sphere_circle.center)),
            Err(e) => Err(e)
        }
    } else {
        circle_circle(&sphere_circle, circle)
    }
}

/// TODO: the parallel case is not handled
/// Calculate the intersection of a line and a plane
/// Line is defined by two points
pub fn plane_line(plane: &Plane, a: &Vec3d, b: &Vec3d) -> Vec3d {
    let ba = b - a;
    let t = (plane.distance - plane.normal.x * a.x + plane.normal.y * a.y + plane.normal.z * a.z)
        / (plane.normal.x * ba.x + plane.normal.y * ba.y + plane.normal.z * ba.z);
    a + t * (b - a)
}

/// Calculate if a point intersects a circle
/// if inner is set to true then points inside the circle are true
/// if inner is set to false then points must lie on the circle edge
/// this function checks if the point and circle lie in the same plane
/// use `circle_point_unchecked()` if this is not needed
/// # Errors
/// `IntersectionError` is returned as appropriate
pub fn circle_point(circle: &Circle, point: &Vec3d, inner: bool) -> Result<(), IntersectionError> {
    let plane = circle.get_plane();
    if plane.contains_point(point) {
        circle_point_unchecked(circle, point, inner)
    } else {
        Err(IntersectionError::OutOfPlane)
    }
}

/// Calculate if a point intersects a circle
/// if inner is set to true then points inside the circle are true
/// if inner is set to false then points must lie on the circle edge
/// this function does not check if the point and circle lie in the same plane
/// because the radius and point distance are compared this is effectively a sphere
/// # Errors
/// `IntersectionError` is returned as appropriate
pub fn circle_point_unchecked(
    circle: &Circle,
    point: &Vec3d,
    inner: bool
) -> Result<(), IntersectionError> {
    let distance = point.distance_to(&circle.center);
    let delta = circle.radius - distance;
    if delta.abs() < f64::EPSILON {
        Ok(())
    } else if delta > f64::EPSILON {
        if inner {
            Ok(())
        } else {
            Err(IntersectionError::ContainedWithin)
        }
    } else {
        Err(IntersectionError::TooFarApart)
    }
}

/// Calculate if a point intersects a plane
/// if inner is set to true then points inside the sphere are true
/// if inner is set to false then points must lie on the sphere edge
/// # Errors
/// `IntersectionError` is returned as appropriate
pub fn sphere_point(sphere: &Sphere, point: &Vec3d, inner: bool) -> Result<(), IntersectionError> {
    let distance = point.distance_to(&sphere.center);
    let delta = sphere.radius - distance;
    if delta.abs() < f64::EPSILON {
        Ok(())
    } else if delta > f64::EPSILON {
        if inner {
            Ok(())
        } else {
            Err(IntersectionError::ContainedWithin)
        }
    } else {
        Err(IntersectionError::TooFarApart)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::angle::AngleRadians;
    use crate::geometry::circle::Circle;
    use crate::geometry::sphere::Sphere;
    use crate::vec3d::Vec3d;

    #[test]
    fn test_sphere_sphere_intersection() {
        let center1 = Vec3d::new(1.0, 1.0, 1.0);
        let center2 = Vec3d::new(-1.0, 1.0, 1.0);
        let center3 = Vec3d::new(0.0, 0.0, -1.0);
        let sphere1 = Sphere::new(&center1, 1.0);
        let sphere2 = Sphere::new(&center2, 1.0);
        let sphere3 = Sphere::new(&center3, 2.0);
        let sphere4 = Sphere::new(&-center3, 2.0);
        let sphere5 = Sphere::new(&center3, 0.2);
        let fudge = 0.000_000_000_000_000_2;
        assert_eq!(
            sphere_sphere(&sphere1, &sphere2),
            Ok(Circle::new(&Vec3d::new(0.0, 1.0, 1.0), 0.0, &Vec3d::i()))
        );
        assert_eq!(
            sphere_sphere(&sphere5, &sphere1),
            Err(IntersectionError::TooFarApart)
        );
        assert_eq!(
            sphere_sphere(&sphere3, &sphere4),
            Ok(Circle::new(
                &Vec3d::zero(),
                2.0 * AngleRadians::sixth_pi().cos() - fudge,
                &Vec3d::k()
            ))
        );
    }

    #[test]
    fn test_sphere_plane_intersection() {
        let center = Vec3d::new(0.0, 0.0, 1.0);
        let sphere = Sphere::new(&center, 1.0);
        let plane1 = Plane::new(&Vec3d::k(), 0.0);
        let plane2 = Plane::new(&Vec3d::k(), 1.0);
        assert_eq!(
            sphere_plane(&sphere, &plane1),
            Ok(Circle::new(&Vec3d::new(0.0, 0.0, 0.0), 0.0, &Vec3d::k()))
        );
        assert_eq!(
            sphere_plane(&sphere, &plane2),
            Ok(Circle::new(&Vec3d::new(0.0, 0.0, 1.0), 1.0, &Vec3d::k()))
        );
    }

    #[test]
    fn test_circle_circle_intersection() {
        let center1 = Vec3d::new(0.0, 0.0, 1.0);
        let center2 = Vec3d::new(0.0, 0.0, 0.0);
        let circle1 = Circle::new(&center1, 1.0, &Vec3d::i());
        let circle2 = Circle::new(&center2, 1.0, &Vec3d::i());
        let circle3 = Circle::new(&center2, 1.0, &Vec3d::j());
        assert_eq!(
            circle_circle(&circle1, &circle2),
            Ok((
                Vec3d::new(0.0, 3.0_f64.sqrt() / -2.0, 0.5),
                Vec3d::new(0.0, 3.0_f64.sqrt() / 2.0, 0.5)
            ))
        );
        assert_eq!(
            circle_circle(&circle1, &circle3),
            Err(IntersectionError::OutOfPlane)
        );
    }

    #[test]
    fn test_sphere_circle_intersection() {
        let center = Vec3d::new(0.0, 0.0, 1.0);
        let sphere = Sphere::new(&center, 1.0);
        let circle1 = Circle::new(&Vec3d::new(1.0, 0.0, 0.0), 1.0, &Vec3d::k());
        let circle2 = Circle::new(&Vec3d::new(1.0, 0.0, 1.0), 1.0, &Vec3d::k());
        // let fudge = 0.000_000_000_000_000_1;
        assert_eq!(
            sphere_circle(&sphere, &circle1),
            Ok((Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 0.0)))
        );
        assert_eq!(
            sphere_circle(&sphere, &circle2),
            Ok((
                Vec3d::new(
                    AngleRadians::third_pi().cos(),
                    -AngleRadians::third_pi().sin(),
                    1.0
                ),
                Vec3d::new(
                    AngleRadians::third_pi().cos(),
                    AngleRadians::third_pi().sin(),
                    1.0
                )
            ))
        );
    }
}
