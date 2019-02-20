use super::circle::Circle;
use super::rectangle::Rectangle;

pub fn is_intersect(rectangle: &Rectangle, circle: &Circle) -> bool {
    // https://stackoverflow.com/a/402019

    let [a, b, c, d] = rectangle.get_transformed_vertices();

    rectangle.is_intersect_point(circle.center)
        || circle.is_intersect_line(a, b)
        || circle.is_intersect_line(b, c)
        || circle.is_intersect_line(c, d)
        || circle.is_intersect_line(d, a)
}
