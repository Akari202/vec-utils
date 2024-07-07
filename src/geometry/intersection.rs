use crate::geometry::circle::Circle;
use crate::geometry::plane::Plane;
use crate::geometry::sphere::Sphere;
use crate::vec3d::Vec3d;

/// Calculate the intersection of two spheres
/// Returns the circle of intersection
/// if the spheres are identical None is returned
/// or None if the spheres do not intersect either because they are too far apart or one is contained within the other
/// if the circles touch at a single point a degenerate circle is returned
pub fn sphere_sphere_intersection(sphere1: &Sphere, sphere2: &Sphere) -> Option<Circle> {
    if sphere1 == sphere2 {
        return None;
    }
    let center_distance = sphere1.center.distance_to(&sphere2.center);
    let radius_sum = sphere1.radius + sphere2.radius;
    if center_distance > radius_sum {
        return None;
    }
    let radius_diff = (sphere1.radius - sphere2.radius).abs();
    if center_distance < radius_diff {
        return None;
    }
    // let circle_radius = (sphere1.radius.powi(2) - sphere2.radius.powi(2) + center_distance.powi(2)) / (2.0 * center_distance);
    // let circle_center = sphere1.center + (sphere2.center - sphere1.center) * (circle_radius / center_distance);

    let h = 0.5 + (sphere1.radius.powi(2) - sphere2.radius.powi(2)) / (2.0 * center_distance.powi(2));
    let radius_of_intersection = (sphere1.radius.powi(2) - h.powi(2) * center_distance.powi(2)).sqrt();
    let center_of_intersection = sphere1.center + h * (sphere2.center - sphere1.center);
    let circle_normal = (sphere2.center - sphere1.center).normalize();
    Some(Circle::new(&center_of_intersection, radius_of_intersection, &circle_normal))
}

/// Calculate the intersection of a sphere and a plane
/// Returns the circle of intersection
/// or None if the sphere does not intersect the plane
pub fn sphere_plane_intersection(sphere: &Sphere, plane: &Plane) -> Option<Circle> {
    let distance = plane.distance_to_point(&sphere.center);
    if distance.abs() > sphere.radius {
        println!("distance: {}", distance);
        return None;
    }
    if distance.abs() == sphere.radius {
        let circle_center = sphere.center - plane.normal * distance;
        return Some(Circle::new(&circle_center, 0.0, &plane.normal));
    }
    let circle_radius = (sphere.radius.powi(2) - distance.powi(2)).sqrt();
    let circle_center = sphere.center + plane.normal * distance;
    Some(Circle::new(&circle_center, circle_radius, &plane.normal))
}

/// Calculate the intersection of two circles
/// DOES NOT CALCULATE INTERSECTION BETWEEN OUT OF PLANE CIRCLES
/// Returns one, two, or no points of intersection
/// The points are returned as a tuple of two Vec3d
/// If the circles do not intersect, None is returned
/// If the circles intersect at one point, the same point is returned twice
/// If the circles intersect at two points,
/// if the circles are identical and have infinite points of intersection, None is returned
pub fn circle_circle_intersection(circle1: &Circle, circle2: &Circle) -> Option<(Vec3d, Vec3d)> {
    if circle1 == circle2 {
        return None;
    }
    if !circle1.in_same_plane(circle2) {
        return None;
    }
    let center_distance = circle1.center.distance_to(&circle2.center);
    let radius_sum = circle1.radius + circle2.radius;
    if center_distance > radius_sum {
        return None;
    }
    let radius_diff = (circle1.radius - circle2.radius).abs();
    if center_distance < radius_diff {
        return None;
    }
    let h = 0.5 + (circle1.radius.powi(2) - circle2.radius.powi(2)) / (2.0 * center_distance.powi(2));
    let radius_of_intersection = (circle1.radius.powi(2) - h.powi(2) * center_distance.powi(2)).sqrt();
    let t = (circle2.center - circle1.center).cross(&circle2.normal).normalize();
    let center_of_intersection = circle1.center + h * (circle2.center - circle1.center);
    let point1 = center_of_intersection + t * radius_of_intersection;
    let point2 = center_of_intersection - t * radius_of_intersection;
    Some((point1, point2))
}

/// Calculate the intersection of a sphere and a circle
/// Returns none if there is no intersection or the intersection is the entire circle
/// if there is one point of intersection it is returned twice
pub fn sphere_circle_intersection(sphere: &Sphere, circle: &Circle) -> Option<(Vec3d, Vec3d)> {
    let circle_plane = circle.get_plane();
    let sphere_circle = sphere_plane_intersection(sphere, &circle_plane)?;
    if sphere_circle.is_degenerate() {
        let intersection_distance = circle.center.distance_to(&sphere_circle.center);
        return if intersection_distance == circle.radius {
            Some((sphere_circle.center, sphere_circle.center))
        } else {
            None
        }
    }
    circle_circle_intersection(&sphere_circle, circle)
}

#[cfg(test)]
mod tests {
    use crate::angle::AngleRadians;
    use crate::geometry::circle::Circle;
    use crate::geometry::sphere::Sphere;
    use crate::vec3d::Vec3d;
    use super::*;

    #[test]
    fn test_sphere_sphere_intersection() {
        let center1 = Vec3d::new(1.0, 1.0, 1.0);
        let center2 = Vec3d::new(-1.0, 1.0, 1.0);
        let center3 = Vec3d::new(0.0, 0.0, -1.0);
        let sphere1 = Sphere::new(&center1, 1.0);
        let sphere2 = Sphere::new(&center2, 1.0);
        let sphere3 = Sphere::new(&center3, 2.0);
        let sphere4 = Sphere::new(&-center3, 2.0);
        let floating_point_error = 0.0000000000000002;
        assert_eq!(
            sphere_sphere_intersection(&sphere1, &sphere2).unwrap(),
            Circle::new(
                &Vec3d::new(0.0, 1.0, 1.0),
                0.0,
                &Vec3d::i()
            )
        );
        assert_eq!(
            sphere_sphere_intersection(&sphere3, &sphere4).unwrap(),
            Circle::new(
                &Vec3d::zero(),
                2.0 * AngleRadians::sixth_pi().cos() - floating_point_error,
                &Vec3d::k()
            )
        );
    }

    #[test]
    fn test_sphere_plane_intersection() {
        let center = Vec3d::new(0.0, 0.0, 1.0);
        let sphere = Sphere::new(&center, 1.0);
        let plane1 = Plane::new(&Vec3d::k(), 0.0);
        let plane2 = Plane::new(&Vec3d::k(), 1.0);
        assert_eq!(
            sphere_plane_intersection(&sphere, &plane1).unwrap(),
            Circle::new(
                &Vec3d::new(0.0, 0.0, 0.0),
                0.0,
                &Vec3d::k()
            )
        );
        assert_eq!(
            sphere_plane_intersection(&sphere, &plane2).unwrap(),
            Circle::new(
                &Vec3d::new(0.0, 0.0, 1.0),
                1.0,
                &Vec3d::k()
            )
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
            circle_circle_intersection(&circle1, &circle2).unwrap(),
            (
                Vec3d::new(0.0, 3.0_f64.sqrt() / -2.0, 0.5),
                Vec3d::new(0.0, 3.0_f64.sqrt() / 2.0, 0.5)
            )
        );
        assert_eq!(
            circle_circle_intersection(&circle1, &circle3),
            None
        );
    }

    #[test]
    fn test_sphere_circle_intersection() {
        let center = Vec3d::new(0.0, 0.0, 1.0);
        let sphere = Sphere::new(&center, 1.0);
        let circle1 = Circle::new(&Vec3d::new(1.0, 0.0, 0.0), 1.0, &Vec3d::k());
        let circle2 = Circle::new(&Vec3d::new(1.0, 0.0, 1.0), 1.0, &Vec3d::k());
        assert_eq!(
            sphere_circle_intersection(&sphere, &circle1).unwrap(),
            (
                Vec3d::new(0.0, 0.0, 0.0),
                Vec3d::new(0.0, 0.0, 0.0)
            )
        );
        assert_eq!(
            sphere_circle_intersection(&sphere, &circle2),
            None
        );
    }
}
