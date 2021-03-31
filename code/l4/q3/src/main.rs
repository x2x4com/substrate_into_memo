
fn main() {
    let c1 = Circle{ radius: 1.2 };
    let s1 = Square{ side: 22.1 };
    let r1 = Rectangle{ long_side:4.12, sort_side: 1.33};
    let t1 = Triangle{ base:15.111, high: 6.0};
    display_area(c1);
    display_area(s1);
    display_area(r1);
    display_area(t1);
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    side: f64,
}

impl CalcArea for Square {
    fn calc_area(&self) -> f64 {
        self.side * self.side
    }
}

struct Rectangle {
    long_side: f64,
    sort_side: f64
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.long_side * self.sort_side
    }
}

struct Triangle {
    base: f64,
    high: f64
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f64 {
        self.base * self.high * 0.5
    }
}

fn display_area<T: CalcArea>(t: T) {
    println!("Area: {}" ,t.calc_area())
}



