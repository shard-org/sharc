use std::marker::PhantomData;
use std::fmt::Debug;

pub struct LinkedList<T> {
   pub front: Link<T>,
   back: Link<T>,
   len: usize,
   _boo: PhantomData<T>,
}

type Link<T> = Option<*mut Node<T>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) { unsafe {
        let new = Box::into_raw(Box::new(Node {
            back: None,
            front: None,
            elem,
        }));

        match self.back {
            Some(old) => {
                (*old).back = Some(new);
                (*new).front = Some(old);
            }
            None => self.front = Some(new),
        }

        self.back = Some(new);
        self.len += 1;
    } }

    pub fn pop_front(&mut self) -> Option<T> { unsafe {
        self.front.map(|node| {
            let boxed_node = Box::from_raw(node);
            let result = boxed_node.elem;

            self.front = boxed_node.back;
            match self.front {
                Some(new) => (*new).front = None,
                None => self.back = None,
            }

            self.len -= 1;
            result
        })
    } }

    pub fn pop_back(&mut self) -> Option<T> { unsafe {
        self.back.map(|node| {
            let boxed_node = Box::from_raw(node);
            let result = boxed_node.elem;

            self.back = boxed_node.front;
            match self.back {
                Some(new) => (*new).back = None,
                None => self.front = None,
            }
            self.len -= 1;
            result
        })
    } }


    pub fn front_mut(&self) -> Option<*mut Node<T>> {
        self.front
    }

    pub fn front(&self) -> Option<&T> {
        self.front.map(|node| unsafe { (*node).elem() })
    }

    pub unsafe fn remove(&mut self, node: *mut Node<T>) -> Node<T> {
        match (*node).back {
            Some(prev) => (*prev).front = (*node).front,
            None => self.front = (*node).front,
        }

        match (*node).front {
            Some(next) => (*next).back = (*node).back,
            None => self.back = (*node).back,
        }

        self.len -= 1;
        *Box::from_raw(node)
    }

    pub fn next(&self, node: *mut Node<T>) -> Option<*mut Node<T>> {
        unsafe { (*node).back }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Node<T> {
    pub fn elem(&self) -> &T {
        &self.elem
    }

    pub fn into_elem(self) -> T {
        self.elem
    }
    
    pub fn next(&self) -> Option<*mut Node<T>> {
        self.back.map(|node| unsafe { node })
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().fold(LinkedList::new(), |mut list, elem| {
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
                write!(f, "{:?}", (*n).elem)?;
                node = (*n).back;
                if node.is_some() {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, "]")
    }
}
