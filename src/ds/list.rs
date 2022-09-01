use std::rc::Rc;

pub struct PerList<T> {
    head: PerLink<T>,
}

type PerLink<T> = Option<Rc<PerNode<T>>>;

struct PerNode<T> {
    elem: T,
    next: PerLink<T>,
}

impl<T> PerList<T> {
    pub fn new() -> Self {
        PerList { head: None }
    }

    pub fn prepend(&self, elem: T) -> PerList<T> {
        PerList {
            head: Some(Rc::new(PerNode {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> PerList<T> {
        PerList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> PerIter<'_, T> {
        PerIter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for PerList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct PerIter<'a, T> {
    next: Option<&'a PerNode<T>>,
}

impl<'a, T> Iterator for PerIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
