/// Must satisfy identity of indiscernibles, symmetry and triangle inequality (https://en.wikipedia.org/wiki/Metric_space)

pub trait Point {
    fn distance(&self, other: &Self) -> f64;
}

pub struct Ball<T: Point> {
    id: usize,
    center: T,
    radio: f64,
}

impl<T: Point> Ball<T> {
    fn new(id: usize, center: T, radio: f64) -> Ball<T> {
        Ball { id, center, radio }
    }

    fn minimum_distance(&self, other: &Self) -> f64 {
        let d = self.center.distance(&other.center);
        let sum_radios = self.radio + other.radio;
        if d < sum_radios { 0.0 } else { d - sum_radios }
    }
}

struct RouterNode<T: Point> {
    ball: Ball<T>,
    left: Node<T>,
    right: Node<T>,
}

struct LeafNode<'a, T: Point> {
    ball: &'a Ball<T>,
}

enum Node<T: Point> {
    RouterNode(T),
    LeafNode(T),
}

struct BallTree<T: Point> {
    root: Node<T>
}

//impl<T:Point> BallTree<T> {
//    fn new(mut balls: Vec<Ball<T>>) -> BallTree<T> {
//        BallTree {
//            root: LeafNode { ball: balls.pop() },
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct R2Point {
        x: f64,
        y: f64,
    }

    impl R2Point {
        fn new(x: f64, y: f64) -> R2Point {
            R2Point { x, y }
        }
    }

    impl Point for R2Point {
        fn distance(&self, other: &Self) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    #[test]
    fn R2_distance() {
        let p1 = R2Point::new(0.0, 0.0);
        let p2 = R2Point::new(0.0, 1.0);

        assert_eq!(p1.distance(&p2), 1.0)
    }

    #[test]
    fn ball_minimum_distance_when_overlapped() {
        let b1 = Ball::new(0, R2Point::new(0.0, 0.0), 1.0);
        let b2 = Ball::new(1, R2Point::new(1.0, 0.0), 1.0);

        assert_eq!(b1.minimum_distance(&b2), 0.0)
    }

    #[test]
    fn ball_minimum_distance_when_not_overlapped() {
        let b1 = Ball::new(0, R2Point::new(0.0, 0.0), 1.0);
        let b2 = Ball::new(1, R2Point::new(3.0, 0.0), 1.0);

        assert_eq!(b1.minimum_distance(&b2), 1.0)
    }

}
