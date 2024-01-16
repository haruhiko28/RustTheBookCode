struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>  {
    fn x(&self) -> &T {
        &self.x
    }

       fn y(&self) -> &T {
        &self.y
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
