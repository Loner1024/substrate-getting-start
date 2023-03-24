use std::f64::consts::PI;

trait Area {
    fn calc_area(&self) -> f64;
}

struct Circle {
    radius: f64
}

impl Area for Circle {
    fn calc_area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64
}

impl Area for Triangle {
    fn calc_area(&self) -> f64 {
        self.base * self.height * 0.5
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn calc_area(&self) -> f64 {
        self.side * self.side
    }
}

fn area<T: Area>(shape: T) -> f64 {
    shape.calc_area()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let circle = Circle { radius: 2.0 };
        assert_eq!(area(circle), PI*2.0*2.0);

        let triangle = Triangle { base: 3.0, height: 4.0 };
        assert_eq!(area(triangle), 6.0);

        let square = Square { side: 2.0 };
        assert_eq!(area(square), 4.0)
    }
}