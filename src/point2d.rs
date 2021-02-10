#[derive(Copy, Clone, Debug)]
pub struct Point2D<T>(pub T, pub T);

impl<T> std::fmt::Display for Point2D<T> where T:std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> Point2D<T> where T:Default {
    pub fn new() -> Point2D<T> {
        Point2D(T::default(), T::default())
    }
    pub fn point(x:T, y:T) -> Point2D<T> {
        Point2D(x, y)
    }
    pub fn add(&self, point:&Point2D<T>) -> Point2D<T> where T: Copy + std::ops::Add<Output = T> {
        Point2D::point(self.0 + point.0, self.1 + point.1)
    }
    pub fn mut_add(&mut self, point:&Point2D<T>) where T: Copy + std::ops::Add<Output = T> {
        self.0 = self.0 + point.0;
        self.1 = self.1 + point.1;
    }
}

impl<T> From<(T,T)> for Point2D<T> {
    fn from(v: (T,T)) -> Point2D<T> {
        Point2D(v.0, v.1)
    }
}

impl Point2D<f64> {
    pub fn mut_rotate_around_origin(&mut self, v:f64) {
        let d: f64 = v.to_radians();
        let x = self.0;
        let y = self.1;

        let x2 = x * d.cos() - y * d.sin();
        let y2 = y * d.cos() + x * d.sin();

        self.0 = x2;
        self.1 = y2;
    }

    pub fn mut_round(&mut self) {
        self.0 = self.0.round();
        self.1 = self.1.round();
    }
}

// let heading = (self.heading as f64).to_radians();
// let dlatitude = -(heading.sin() * (*v as f64));
// let dlongitude = heading.cos() * (*v as f64);

// self.latitude += dlatitude as i16;
// self.longitude += dlongitude as i16;

#[cfg(test)]
mod tests {
    use super::Point2D;

    #[test]
    fn test_point2d_new() {
        let _point: Point2D<i32> = Point2D::new();
    }
    #[test]
    fn test_point2d_new2() {
        let point: Point2D<f32> = Point2D::point(0.2, 0.4);
        println!("{}", point);
    }

    #[test]
    fn test_point2d_into() {
        let point: Point2D<_> = (0.2, 0.4).into();
        println!("{}", point);
    }

    #[test]
    fn test_point2d_add() {
        let mut point1: Point2D<i32> = Point2D::new();
        let point2: Point2D<i32> = Point2D::point(1,2);
        point1.mut_add(&point2);
        println!("{}", &point1);
        let point3: Point2D<i32> = Point2D::point(1,2);
        let point4 = point1.add(&point3);
        println!("{}", &point4);
    }

    #[test]
    fn test_point2d_rotate() {
        let mut point: Point2D<f64> = Point2D::point(1.0,2.0);
        println!("{}", &point);
        point.mut_rotate_around_origin(90.0);
        println!("{}", &point);
        point.mut_round();
        println!("{}", &point);
    }

    #[test]
    fn test_point2d_borrow() {
        let point1: Point2D<f64> = Point2D::point(1.0,2.0);
        let point2: Point2D<f64> = Point2D::point(2.0,1.0);
        let point = point1.add(&point2);
        println!("{} {} {}", &point, &point1, &point2);
    }
}
