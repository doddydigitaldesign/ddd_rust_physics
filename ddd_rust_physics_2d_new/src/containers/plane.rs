#[derive(Debug, PartialEq)]
pub struct Plane {
    pub size: (f64, f64),
}

impl Plane {
    pub fn new(size_x: f64, size_y: f64) -> Plane {
        Plane {
            size: (size_x, size_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    #[test]
    fn it_works() {
        let plane = Plane::new(10.0, 10.0);

        assert_eq!(plane.size.0, 10.0);
        assert_eq!(plane.size.1, 10.0);
    }
}
