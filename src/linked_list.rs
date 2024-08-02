use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct LinkedList<T> {
    front:   Link<T>,
    back:    Link<T>,
    current: Link<T>,
    len:     usize,
    _boo:    PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { 
            front: None,
            back: None,
            current: None,
            len: 0,
            _boo: PhantomData
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node { prev: None, next: None, elem })));

            match self.back {
                Some(old) => {
                    (*old.as_ptr()).next = Some(new);
                    (*new.as_ptr()).prev = Some(old);
                },
                None => {
                    self.front   = Some(new);
                    self.current = Some(new);
                },
            }

            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.map(|node| {
            unsafe {
                let node = Box::from_raw(node.as_ptr());

                match node.next {
                    Some(next) => (*next.as_ptr()).prev = None,
                    None => self.back = None,
                }

                self.front = node.next;
                self.len -= 1;
                node.elem
            }
        })
    }

    pub fn front(&self) -> Option<&T> {
        self.front.map(|node| unsafe{ &node.as_ref().elem })
    }

    pub fn advance(&mut self) {
        self.current.map(|node| unsafe{ self.current = node.as_ref().next; });
    }

    pub fn current(&self) -> Option<&T> {
        self.current.map(|node| unsafe{ &node.as_ref().elem })
    }

    pub fn peek(&self) -> Option<&T> {
        self.current.and_then(|node| unsafe{ node.as_ref().next })
            .map(|n| unsafe{ &n.as_ref().elem })
    }

    pub fn consume(&mut self) -> Option<T> {
        self.current.map(|node| {
            unsafe {
                let node = Box::from_raw(node.as_ptr());

                match node.prev {
                    Some(prev) => (*prev.as_ptr()).next = node.next,
                    None => self.front = node.next,
                }

                match node.next {
                    Some(next) => (*next.as_ptr()).prev = node.prev,
                    None => self.back = node.prev,
                }

                let elem = node.elem;
                self.current = node.next;
                self.len -= 1;
                elem
            }
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().fold(Self::new(), |mut list, elem| {
            list.push(elem);
            list
        })
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut node = self.front;
        write!(f, "[")?;
        while let Some(n) = node {
            unsafe {
                write!(f, "{:?}", n.as_ref().elem)?;
                node = n.as_ref().next;
                if node.is_some() {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, "]")
    }
}
