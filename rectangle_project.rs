// Define a struct `Rectangle` with width and height as floating-point numbers (f64).
struct Rectangle {
    width: f64,
    height: f64,
}

// Implement methods for the Rectangle struct
impl Rectangle {
    // Associated function to create a new Rectangle instance
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Method to calculate the area of the rectangle
    fn area(&self) -> f64 {
        // Area formula: width * height
        self.width * self.height
    }

    // Method to calculate the perimeter of the rectangle
    fn perimeter(&self) -> f64 {
        // Perimeter formula: 2 * (width + height)
        2.0 * (self.width + self.height)
    }

    // Method to check if the rectangle is a square
    fn is_square(&self) -> bool {
        // A rectangle is a square if width == height
        self.width == self.height
    }
}

fn main() {
    // Create a rectangle with width = 10.0 and height = 5.0
    let rect = Rectangle::new(10.0, 5.0);

    // Print out area, perimeter, and whether it is a square
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    // Test cases using assert macros
    // This should pass because 5x5 is a square
    assert!(Rectangle::new(5.0, 5.0).is_square());

    // This should pass because 5x6 is not a square
    assert!(!Rectangle::new(5.0, 6.0).is_square());
}
