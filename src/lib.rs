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

    impl Point for R2Point {
        fn distance(&self, other: &Self) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    #[test]
    fn R2_distance() {
        let p1 = R2Point { x: 0.0, y: 0.0 };
        let p2 = R2Point { x: 0.0, y: 1.0 };

        assert_eq!(p1.distance(&p2), 1.0)
    }
}
