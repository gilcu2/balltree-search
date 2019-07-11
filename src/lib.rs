/// Must satisfy identity of indiscernibles, symmetry and triangle inequality (https://en.wikipedia.org/wiki/Metric_space)

pub trait Metric<T> {
    fn distance(o1: &T, o2: &T) -> f64;
}

pub type Id = usize;
pub type Real = f64;

pub struct Ball<T> {
    id: Id,
    center: T,
    radio: Real,
}

impl<T> Ball<T> {
    fn new(id: Id, center: T, radio: Real) -> Ball<T> {
        Ball { id, center, radio }
    }
}

struct RouterNode<T> {
    ball: Ball<T>,
    left: Node<T>,
    right: Node<T>,
}

struct LeafNode<T> {
    ball: &Ball<T>,
}

enum Node<T> {
    RouterNode(T),
    LeafNode(T),
}

struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    fn new(mut balls: Vec<Ball<T>>, metric: dyn Metric<T>) -> Ball<T> {
        Tree {
            root: LeafNode { ball: balls.pop() },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
