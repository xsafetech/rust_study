use std::f64::consts::PI;

// Define a struct for representing a point in 2D space
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    // Method to calculate the distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}


// Define an enum to represent different types of shapes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Circle { center: Point, radius: f64},
    Rectangle { top_left: Point, bottom_right: Point },
    Triangle { p1: Point, p2: Point, p3: Point},
}

// Define a trait for shapes that can calculate their area
pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } =>  PI * radius * radius,
            Shape::Rectangle { top_left, bottom_right } => {
                let width = (bottom_right.x - top_left.x).abs();
                let height = (bottom_right.y - top_left.y).abs();
                width * height
            }
            Shape::Triangle { p1, p2, p3 } => {
                // Heron's Formula
                let a = p1.distance(p2);
                let b = p2.distance(p3);
                let c = p3.distance(p1);
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// Define a trait for shapes that can calculate their perimeter
pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

impl Perimeter for Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => 2.0 * PI * radius,
              Shape::Rectangle { top_left, bottom_right } => {
                  let width = (bottom_right.x - top_left.x).abs();
                  let height = (bottom_right.y - top_left.y).abs();
                  2.0 * (width + height)
              }
           Shape::Triangle { p1, p2, p3 } => {
                  let a = p1.distance(p2);
                  let b = p2.distance(p3);
                  let c = p3.distance(p1);
                  a + b + c
             }
          }
    }
}