enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 50,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let red = TrafficLight::Red;
        assert_eq!(red.time(), 30);

        let green = TrafficLight::Green;
        assert_eq!(green.time(), 40);

        let yellow = TrafficLight::Yellow;
        assert_eq!(yellow.time(), 50);
    }
}
