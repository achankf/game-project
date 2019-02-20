use nalgebra::Point2;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Point2<f32>,
    pub radius: f32,
}

impl Circle {
    /// line is formed by (a,b), epsilon takes care of arounding errors
    pub fn is_intersect_line_with_epsilon(
        &self,
        a: Point2<f32>,
        b: Point2<f32>,
        epsilon: f32,
    ) -> bool {
        // https://math.stackexchange.com/a/2035466
        // http://mathworld.wolfram.com/Circle-LineIntersection.html
        let Circle { center, radius } = self;

        // translate a and b by c, to simplify the problem to testing a line to a circle centered around the origin
        let ta = a - center;
        let tb = b - center;
        let dr = nalgebra::distance(&a, &b);
        let dr2 = dr * dr;
        let r2 = radius * radius;
        let det = ta.x * tb.y - tb.x * ta.y;
        let det2 = det * det;
        let discriminant = r2 * dr2 - det2;

        // count tangent as intersection
        discriminant >= 0. || discriminant.abs() < epsilon
    }

    pub fn is_intersect_line(&self, a: Point2<f32>, b: Point2<f32>) -> bool {
        const EPSILON: f32 = 0.00001;
        self.is_intersect_line_with_epsilon(a, b, EPSILON)
    }
}

#[cfg(test)]
mod circle_tests {
    use super::*;

    #[test]
    fn unit_circle_line_intersect() {
        let circle = Circle {
            center: Point2::new(1., 1.),
            radius: 1.,
        };

        assert!(circle.is_intersect_line(Point2::new(2., 100.), Point2::new(2., -100.))); // touch on (2,1)
        assert!(circle.is_intersect_line(Point2::new(0., 2.), Point2::new(2., 1.))); // touch on (1,2)
        assert!(circle.is_intersect_line(Point2::new(2., 2.), Point2::new(0., 0.))); // touch on (0,1)
        assert!(circle.is_intersect_line(Point2::new(0., 0.), Point2::new(2., 0.))); // touch on (0,1)

        // touch on (a,a), where a = 1 + sqrt(2) / 2
        let sqrt2 = 2f32.sqrt();
        assert!(circle.is_intersect_line(Point2::new(0., 2. + sqrt2), Point2::new(2. + sqrt2, 0.)));

        // not intersect
        assert!(
            !circle.is_intersect_line(Point2::new(23420., 234230.), Point2::new(23422., 234230.))
        );
    }

    #[test]
    fn unit_circle_line_intersect_tangent_point() {
        let circle = Circle {
            center: Point2::new(0., 0.),
            radius: 10.,
        };
        assert!(circle.is_intersect_line(Point2::new(0., 10.), Point2::new(100., 10.)));
        assert!(circle.is_intersect_line(Point2::new(100., 10.), Point2::new(0., 10.)));
        assert!(circle.is_intersect_line(Point2::new(-10., 0.), Point2::new(234234230., 23420.)));
    }
}
