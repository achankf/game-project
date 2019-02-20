use nalgebra::geometry::Rotation2;
use nalgebra::{Point2, Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub center: Point2<f32>,
    pub length: f32,
    pub width: f32,
    pub angle: f32, // in radian
}

impl Rectangle {
    pub fn get_vertices(&self) -> [Point2<f32>; 4] {
        let Rectangle { length, width, .. } = self;
        let half_width = width / 2.;
        let half_length = length / 2.;
        let a = Point2::new(-half_width, -half_length);
        let b = Point2::new(half_width, -half_length);
        let c = Point2::new(half_width, half_length);
        let d = Point2::new(-half_width, half_length);
        [a, b, c, d]
    }

    /// get vertices clockwise, starting from the top-left
    pub fn get_transformed_vertices(&self) -> [Point2<f32>; 4] {
        let Rectangle { center, angle, .. } = self;

        let rot = Rotation2::new(*angle);
        let translation = Vector2::new(center.x, center.y);

        let [a, b, c, d] = self.get_vertices();

        let a = rot * a + translation;
        let b = rot * b + translation;
        let c = rot * c + translation;
        let d = rot * d + translation;

        [a, b, c, d]
    }

    pub fn is_intersect_point_with_epsilon(&self, point: Point2<f32>, epsilon: f32) -> bool {
        // https://stackoverflow.com/a/2763387
        // 0 <= dot(AB,AM) <= dot(AB,AB) &&
        // 0 <= dot(BC,BM) <= dot(BC,BC)
        let [a, b, c, _] = self.get_transformed_vertices();
        let m = point;

        let ab = b - a;
        let bc = c - b;
        let am = m - a;
        let bm = m - b;

        let ab_dot_am = ab.dot(&am);
        let bc_dot_bm = bc.dot(&bm);

        /*
        println!(
            "point:{}, a:{}, b:{}, c:{}, ab.am:{} (<={}), bc.bm:{} (<={})",
            point,
            a,
            b,
            c,
            ab_dot_am,
            ab.dot(&ab),
            bc_dot_bm,
            bc.dot(&bc)
        ); 
        */

        (ab_dot_am >= 0. || ab_dot_am.abs() <= epsilon)
            && ab_dot_am <= ab.dot(&ab) + epsilon
            && (bc_dot_bm >= 0. || bc_dot_bm.abs() <= epsilon)
            && bc_dot_bm <= bc.dot(&bc) + epsilon
    }

    pub fn is_intersect_point(&self, point: Point2<f32>) -> bool {
        const EPSILON: f32 = 0.00001;
        self.is_intersect_point_with_epsilon(point, EPSILON)
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn square_within_rotation_point_intersect() {
        let rect = Rectangle {
            center: Point2::new(0., 0.),
            length: 10.,
            width: 10.,
            angle: 0.,
        };

        assert!(rect.is_intersect_point(Point2::new(0., 0.)));

        // vertices
        assert!(rect.is_intersect_point(Point2::new(-5., -5.)));
        assert!(rect.is_intersect_point(Point2::new(5., 5.)));
        assert!(rect.is_intersect_point(Point2::new(5., -5.)));
        assert!(rect.is_intersect_point(Point2::new(-5., 5.)));

        // points on edges
        assert!(rect.is_intersect_point(Point2::new(-5., 0.)));
        assert!(rect.is_intersect_point(Point2::new(5., 0.)));
        assert!(rect.is_intersect_point(Point2::new(0., -5.)));
        assert!(rect.is_intersect_point(Point2::new(0., 5.)));

        // points within rect
        assert!(rect.is_intersect_point(Point2::new(2., -2.)));
        assert!(rect.is_intersect_point(Point2::new(1.5235, -4.3463)));

        // outside of rect
        assert!(!rect.is_intersect_point(Point2::new(6., -0.)));
        assert!(!rect.is_intersect_point(Point2::new(6., -6.)));
        assert!(!rect.is_intersect_point(Point2::new(234., -2432.)));
    }

    #[test]
    fn square_with_rotation_point_intersect() {
        let rect = Rectangle {
            center: Point2::new(0., 0.),
            length: 10.,
            width: 10.,
            angle: std::f32::consts::PI,
        };

        assert!(rect.is_intersect_point(Point2::new(0., 0.)));

        // vertices
        assert!(rect.is_intersect_point(Point2::new(-5., -5.)));
        assert!(rect.is_intersect_point(Point2::new(5., 5.)));
        assert!(rect.is_intersect_point(Point2::new(5., -5.)));
        assert!(rect.is_intersect_point(Point2::new(-5., 5.)));

        // points on edges
        assert!(rect.is_intersect_point(Point2::new(-5., 0.)));
        assert!(rect.is_intersect_point(Point2::new(5., 0.)));
        assert!(rect.is_intersect_point(Point2::new(0., -5.)));
        assert!(rect.is_intersect_point(Point2::new(0., 5.)));

        // points within rect
        assert!(rect.is_intersect_point(Point2::new(2., -2.)));
        assert!(rect.is_intersect_point(Point2::new(1.5235, -4.3463)));

        // outside of rect
        assert!(!rect.is_intersect_point(Point2::new(6., -0.)));
        assert!(!rect.is_intersect_point(Point2::new(6., -6.)));
        assert!(!rect.is_intersect_point(Point2::new(234., -2432.)));
    }

    #[test]
    fn rect_without_rotation_point_intersect() {
        let rect = Rectangle {
            center: Point2::new(0., 0.),
            length: 10.,
            width: 5.,
            angle: 0.,
        };

        assert!(rect.is_intersect_point(Point2::new(0., 0.)));

        // vertices
        assert!(rect.is_intersect_point(Point2::new(-2.5, -5.)));
        assert!(rect.is_intersect_point(Point2::new(2.5, 5.)));
        assert!(rect.is_intersect_point(Point2::new(2.5, -5.)));
        assert!(rect.is_intersect_point(Point2::new(-2.5, 5.)));

        // points on edges
        assert!(rect.is_intersect_point(Point2::new(-2.5, 0.)));
        assert!(rect.is_intersect_point(Point2::new(2.5, 0.)));
        assert!(rect.is_intersect_point(Point2::new(0., -5.)));
        assert!(rect.is_intersect_point(Point2::new(0., 5.)));

        // points within rect
        assert!(rect.is_intersect_point(Point2::new(2., -2.)));
        assert!(rect.is_intersect_point(Point2::new(1.5235, -4.3463)));

        // outside of rect
        assert!(!rect.is_intersect_point(Point2::new(-0., 5.1)));
        assert!(!rect.is_intersect_point(Point2::new(2.6, -0.)));
        assert!(!rect.is_intersect_point(Point2::new(6., -6.)));
        assert!(!rect.is_intersect_point(Point2::new(234., -2432.)));
    }

    #[test]
    fn rect_with_angle_rotation_point_intersect() {
        let rect = Rectangle {
            center: Point2::new(0., 0.),
            length: 10.,
            width: 5.,
            angle: std::f32::consts::PI / 2.23423,
        };

        assert!(rect.is_intersect_point(Point2::new(0., 0.)));

        // points whose vertex magnitude are shorter than the rectangle's sides
        assert!(rect.is_intersect_point(Point2::new(-2., -2.)));
        assert!(rect.is_intersect_point(Point2::new(2., 2.)));
        assert!(rect.is_intersect_point(Point2::new(2., -2.)));
        assert!(rect.is_intersect_point(Point2::new(-2., 2.)));

        // before-rotate vertices aren't in the rectangle anymore
        assert!(!rect.is_intersect_point(Point2::new(-2.5, -5.)));
        assert!(!rect.is_intersect_point(Point2::new(2.5, 5.)));
        assert!(!rect.is_intersect_point(Point2::new(2.5, -5.)));
        assert!(!rect.is_intersect_point(Point2::new(-2.5, 5.)));
    }
}
