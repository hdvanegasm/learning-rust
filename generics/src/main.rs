// Hint: don't use too much generic types because it will make the code
// hard to read.

// Generics in structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Multiple generic types with struct
#[derive(Debug)]
struct DualPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic types relevant in the struct definition
impl<T, U> DualPoint<T, U> {
    // Generic types relevant only in the method
    fn mixup<C, D>(self, other: DualPoint<C, D>) -> DualPoint<T, D> {
        DualPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generics in functions
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 75];
    let result = largest(&number_list);
    println!("The largest element in the array is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let point_int = Point {
        x: 5,
        y: 6,
    };

    let point_float = Point {
        x: 5.1,
        y: 6.5,
    };

    println!("Point is {:?}", point_int);
    println!("x coordinate in Point is {}", point_int.x());
    println!("Point is {:?}", point_float);
    println!("The distance from origin of point float is {}", point_float.distance_from_origin());
    
    let dual_point_numeric = DualPoint {
        x: 1,
        y: 2.5,
    };

    let dual_point_char = DualPoint {
        x: 'a',
        y: 'b',
    };

    println!("The dual point_numeric is {:?}", dual_point_numeric);
    println!("The dual point_char is {:?}", dual_point_char);
    let mixed_point = dual_point_numeric.mixup(dual_point_char);
    println!("The dual point_mixed is {:?}", mixed_point);
}
