use geometry::{Area, Perimeter, Point, Shape};

fn main() {
    // Create some points
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 4.0, y: 6.0 };

    // Calculate distance
    let distance = p1.distance(&p2);
    println!("Distance between p1 and p2: {}", distance);

    // Create some shapes
    let circle = Shape::Circle {
        center: p1,
        radius: 5.0,
    };
    let rectangle = Shape::Rectangle {
        top_left: p1,
        bottom_right: p2,
    };
    let triangle = Shape::Triangle {
        p1: Point { x: 0.0, y: 0.0 },
        p2: Point { x: 3.0, y: 0.0 },
        p3: Point { x: 0.0, y: 4.0 },
    };

    // Calculate area
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
    println!("Triangle area: {}", triangle.area());

    // Calculate perimeter
    println!("Circle perimeter: {}", circle.perimeter());
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Triangle perimeter: {}", triangle.perimeter());
}
