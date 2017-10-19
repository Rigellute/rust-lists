use std::mem;

// In first we rolled our own version of the Option type
type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }
    // Retrieve the value at the head
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);

        }
    }

}

#[cfg(test)]
mod test {
    use super::List;
    use super::add_nums_to_list;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push("hello");
        list.push("world");

        assert_eq!(list.pop(), Some("world"));
        assert_eq!(list.pop(), Some("hello"));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = add_nums_to_list(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
    }
    #[test]
    fn into_iter() {
        let list = add_nums_to_list(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
    #[test]
    fn iter() {
        let list = add_nums_to_list(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = add_nums_to_list(3);
        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 3));
    }
}

pub fn add_nums_to_list(upper_bound: i32) -> List<i32> {
    let mut list = List::new();
    for num in 0..(upper_bound + 1) {
        list.push(num);
    };

    list
}
