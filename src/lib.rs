pub mod calc {
    ///```
    /// let result = rusty::calc::celsius2farenheit(10);
    /// assert_eq!(result,50);
    /// ```
    pub fn celsius2farenheit(celsius: i32) -> i32 {
        let f: i32 = (celsius * 9 / 5) + 32;
        f
    }

    ///```
    /// let result = rusty::calc::farenheit2celsius(50);
    /// assert_eq!(result,10);
    /// ```
    pub fn farenheit2celsius(farenheit: i32) -> i32 {
        let c: i32 = (farenheit - 32) * 5 / 9;
        c
    }

    ///```
    /// let result = rusty::calc::fibonacci_loop(10);
    /// assert_eq!(result,55);
    /// ```
    pub fn fibonacci_loop(n: u32) -> u64 {
        //current = sum of prev 2

        let mut prev: u64 = 0;
        let mut current: u64 = 1;

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        for _ in 2..=n {
            let next: u64 = prev + current;
            prev = current;
            current = next;
        }

        current
    }

    ///```
    /// let result = rusty::calc::fibonacci_rec(10);
    /// assert_eq!(result,55);
    /// ```
    pub fn fibonacci_rec(n: u32) -> u64 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::calc;

    #[test]
    fn unit_test_c_to_f() {
        assert_eq!(calc::celsius2farenheit(0), 32);
        assert_eq!(calc::celsius2farenheit(100), 212);
        assert_eq!(calc::celsius2farenheit(-40), -40);
        assert_eq!(calc::celsius2farenheit(37), 98);
    }

    #[test]
    fn unit_test_f_to_c() {
        assert_eq!(calc::farenheit2celsius(32), 0);
        assert_eq!(calc::farenheit2celsius(212), 100);
        assert_eq!(calc::farenheit2celsius(-40), -40);
        assert_eq!(calc::farenheit2celsius(98), 36);
    }

    #[test]
    fn unit_test_fib_loop() {
        assert_eq!(calc::fibonacci_loop(10), 55);
    }

    #[test]
    fn unit_test_fib_rec() {
        assert_eq!(calc::fibonacci_rec(10), 55)
    }
}

pub mod song {
    pub fn print_lyrics() {
        let days = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "eleventh", "twelfth",
        ];

        let gifts = [
            "a Partridge in a Pear Tree",
            "two Turtle Doves",
            "three French Hens",
            "four Calling Birds",
            "five Gold Rings",
            "six Geese a-Laying",
            "seven Swans a-Swimming",
            "eight Maids a-Milking",
            "nine Ladies Dancing",
            "ten Lords a-Leaping",
            "eleven Pipers Piping",
            "twelve Drummers Drumming",
        ];

        for day in 0..12 {
            println!(
                "On the {} day of Christmas\nmy true love sent to me:",
                days[day]
            );

            for gift in (0..=day).rev() {
                if day > 0 && gift == 0 {
                    print!("and ");
                }
                println!("{}", gifts[gift]);
            }

            println!();
        }
    }
}

// src/lib.rs
pub mod figure {
    #[derive(Debug, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum Shape {
        Point(Point),
        Circle {
            center: Point,
            radius: f64,
        },
        Rectangle {
            top_left: Point,
            bottom_right: Point,
        },
        Triangle {
            p1: Point,
            p2: Point,
            p3: Point,
        },
    }

    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Point(_) => 0.0,
                Shape::Circle { radius, .. } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle {
                    top_left,
                    bottom_right,
                } => {
                    let width = (bottom_right.x - top_left.x).abs();
                    let height = (top_left.y - bottom_right.y).abs();
                    width * height
                }
                Shape::Triangle { p1, p2, p3 } => {
                    let a = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
                    let b = ((p2.x - p3.x).powi(2) + (p2.y - p3.y).powi(2)).sqrt();
                    let c = ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2)).sqrt();
                    let s = (a + b + c) / 2.0;
                    (s * (s - a) * (s - b) * (s - c)).sqrt()
                }
            }
        }

        pub fn perimeter(&self) -> f64 {
            match self {
                Shape::Point(_) => 0.0,
                Shape::Circle { radius, .. } => 2.0 * std::f64::consts::PI * radius,
                Shape::Rectangle {
                    top_left,
                    bottom_right,
                } => {
                    let width = (bottom_right.x - top_left.x).abs();
                    let height = (top_left.y - bottom_right.y).abs();
                    2.0 * (width + height)
                }
                Shape::Triangle { p1, p2, p3 } => {
                    let a = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
                    let b = ((p2.x - p3.x).powi(2) + (p2.y - p3.y).powi(2)).sqrt();
                    let c = ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2)).sqrt();
                    a + b + c
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_point_area_perimeter() {
            let point = Shape::Point(Point { x: 0.0, y: 0.0 });
            assert_eq!(point.area(), 0.0);
            assert_eq!(point.perimeter(), 0.0);
        }

        #[test]
        fn test_circle_area_perimeter() {
            let circle = Shape::Circle {
                center: Point { x: 0.0, y: 0.0 },
                radius: 1.0,
            };
            assert_eq!(circle.area(), std::f64::consts::PI);
            assert_eq!(circle.perimeter(), 2.0 * std::f64::consts::PI);
        }

        #[test]
        fn test_rectangle_area_perimeter() {
            let rectangle = Shape::Rectangle {
                top_left: Point { x: 0.0, y: 1.0 },
                bottom_right: Point { x: 1.0, y: 0.0 },
            };
            assert_eq!(rectangle.area(), 1.0);
            assert_eq!(rectangle.perimeter(), 4.0);
        }

        #[test]
        fn test_triangle_area_perimeter() {
            let triangle = Shape::Triangle {
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 1.0, y: 0.0 },
                p3: Point { x: 0.0, y: 1.0 },
            };
            let expected_area = 0.5;
            let expected_perimeter = 2.0 + (2.0_f64).sqrt();
            assert!((triangle.area() - expected_area).abs() < 1e-10);
            assert!((triangle.perimeter() - expected_perimeter).abs() < 1e-10);
        }
    }
}
