use nalgebra::{Point2, Vector2};
use num::Signed;
use ordered_float::OrderedFloat;

const EPSILON: f32 = 0.00001;

// https://stackoverflow.com/a/2249237
// vector version: https://stackoverflow.com/a/32341982

fn predict_intersection_time(
    src_position: Point2<f32>,
    src_velocity: Vector2<f32>,
    projectile_speed: f32,
    target_position: Point2<f32>,
    target_velocity: Vector2<f32>,
) -> Option<OrderedFloat<f32>> {
    debug_assert!(projectile_speed.is_positive());

    let relative_velocity = target_velocity - src_velocity;
    let relative_position = target_position - src_position;

    let p2 = projectile_speed * projectile_speed;

    // quadratic coefficients
    let a = relative_velocity.dot(&relative_velocity) - p2;
    let b = 2. * relative_position.dot(&relative_velocity);
    let c = relative_position.dot(&relative_position);

    if a.abs() < EPSILON {
        // special cases (equation is linear or constant):
        // https://stackoverflow.com/a/3487761
        if b.abs() < EPSILON {
            None // no change in slope, no solution
        } else {
            let solution = -c / b;
            if solution.is_positive() {
                // the equation is linear
                Some(OrderedFloat(solution))
            } else {
                None
            }
        }
    } else {
        // quadratic
        let discriminant = b * b - 4. * a * c;

        if discriminant.is_negative() {
            None
        } else {
            let neg_b = -b;
            let sqrt_discriminant = discriminant.sqrt();
            let two_a = a + a;

            [
                OrderedFloat((neg_b + sqrt_discriminant) / two_a),
                OrderedFloat((neg_b - sqrt_discriminant) / two_a),
            ]
            .into_iter()
            .filter(|OrderedFloat(t)| t.is_positive())
            .min()
            .cloned()
        }
    }
}

pub fn predict_intersection_point(
    src_position: Point2<f32>,
    src_velocity: Vector2<f32>,
    projectile_speed: f32,
    target_position: Point2<f32>,
    target_velocity: Vector2<f32>,
) -> Option<Point2<f32>> {
    assert!(projectile_speed.is_positive());

    predict_intersection_time(
        src_position,
        src_velocity,
        projectile_speed,
        target_position,
        target_velocity,
    )
    .map(|OrderedFloat(t)| {
        let x = t * target_velocity.x + target_position.x;
        let y = t * target_velocity.y + target_position.y;
        Point2::new(x, y)
    })
}
