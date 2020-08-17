use std::cmp::Ordering;
use std::ops::Index;
use std::fmt::{Formatter, Debug};
use std::alloc::{alloc_zeroed, Layout};
use std::mem::{size_of, align_of};

pub type Result<O> = std::result::Result<O, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    ZeroIndex,
    NoParent,
    NoChildren,
    OutOfBounds(usize),
    Custom(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred because of: {:?}", self.kind)
    }
}

pub enum HeapKind {
    Max,
    Min,
}

#[derive(Clone)]
pub struct BinaryHeap<T, Cmp> {
    inner: Vec<T>,
    compare: Cmp,
}

// construct and helper
impl<T, Cmp> BinaryHeap<T, Cmp> {
    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    pub fn swap(&mut self, a: usize, b: usize) -> Result<()> {
        let a = self.check(a)?;
        let b = self.check(b)?;

        unsafe {
            Ok(self.swap_unchecked(a, b))
        }
    }

    fn check(&self, index: usize) -> Result<usize> {
        match index {
            0 => Err(Error { kind: ErrorKind::ZeroIndex }),
            idx => if index < self.inner.len() {
                Ok(idx)
            } else {
                Err(Error { kind: ErrorKind::OutOfBounds(idx) })
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.check(index).ok()?;

        self.inner.get(index)
    }

    pub const unsafe fn from_source_unchecked(source: Vec<T>, compare: Cmp) -> Self {
        BinaryHeap {
            inner: source,
            compare,
        }
    }

    pub fn from_source(source: Vec<T>, compare: Cmp) -> Option<Self> {
        if source.is_empty() {
            None
        } else {
            unsafe {
                Some(Self::from_source_unchecked(source, compare))
            }
        }
    }

    pub fn new(compare: Cmp) -> Self {
        let layout = Layout::from_size_align(size_of::<T>(), align_of::<T>()).unwrap();
        let zeroed = unsafe { alloc_zeroed(layout) as *mut T };
        let unsafe_vec = unsafe { Vec::from_raw_parts(zeroed, 1, 1) };

        BinaryHeap {
            inner: unsafe_vec,
            compare,
        }
    }

    pub fn with_capacity(capacity: usize, compare: Cmp) -> Self {
        BinaryHeap {
            inner: Vec::with_capacity(capacity),
            compare,
        }
    }

    pub fn parent(&self, idx: usize) -> Result<usize> {
        self.check(idx)?;

        match idx {
            1 => Err(Error { kind: ErrorKind::NoParent }),
            idx => Ok(idx / 2)
        }
    }

    pub fn children(&self, idx: usize) -> Result<(Option<usize>, Option<usize>)> {
        self.check(idx)?;

        let lc = self.check(idx * 2).ok();
        let rc = self.check(idx * 2 + 1).ok();

        Ok((lc, rc))
    }

    pub fn end(&self) -> usize {
        self.inner.len() - 1
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub unsafe fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T, Cmp> Index<usize> for BinaryHeap<T, Cmp> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

// insert
impl<T, Cmp> BinaryHeap<T, Cmp>
    where Cmp: Fn(&T, &T) -> Ordering {
    fn sink(&mut self, target: usize) -> Result<()> {
        self.check(target)?;

        todo!()
    }

    fn float(&mut self, target: usize) -> Result<()> {
        let target = self.check(target)?;

        if let Some(parent) = self.parent(target).ok() {
            match (self.compare)(&self[target], &self[parent]) {
                Ordering::Greater => unsafe {
                    self.swap_unchecked(target, parent);
                    self.float(parent)?;
                }

                _ => ()
            }
        }

        Ok(())
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);

        self.float(self.end());
    }
}

impl <T: Debug, Cmp> Debug for BinaryHeap<T, Cmp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::bheap::BinaryHeap;
    use std::cmp::Ordering;

    fn construct() -> BinaryHeap<u32, ()> {
        BinaryHeap::from_source(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], ()).unwrap()
    }

    #[test]
    fn parent() {
        let heap = construct();

        assert_eq!(None, heap.parent(0).ok());
        assert_eq!(None, heap.parent(1).ok());
        assert_eq!(Some(1), heap.parent(2).ok());
        assert_eq!(Some(1), heap.parent(3).ok());
        assert_eq!(Some(2), heap.parent(4).ok());
        assert_eq!(Some(2), heap.parent(5).ok());
        assert_eq!(Some(3), heap.parent(6).ok());
        assert_eq!(Some(3), heap.parent(7).ok());
        assert_eq!(Some(4), heap.parent(8).ok());
        assert_eq!(None, heap.parent(9).ok());
    }

    #[test]
    fn children() {
        let heap = construct();

        assert_eq!(None, heap.children(0).ok());
        assert_eq!(Some((Some(2), Some(3))), heap.children(1).ok());
        assert_eq!(Some((Some(4), Some(5))), heap.children(2).ok());
        assert_eq!(Some((Some(6), Some(7))), heap.children(3).ok());
        assert_eq!(Some((Some(8), None)), heap.children(4).ok());
        assert_eq!(Some((None, None)), heap.children(5).ok());
    }

    #[test]
    fn push() {
        let mut heap: BinaryHeap<u32, fn(&u32, &u32) -> Ordering> = BinaryHeap::new(|a, b| u32::cmp(a, b));

        heap.push(5);
        assert_eq!(&vec![0, 5], heap.inner());
        heap.push(2);
        assert_eq!(&vec![0, 5, 2], heap.inner());
        heap.push(3);
        assert_eq!(&vec![0, 5, 2, 3], heap.inner());
        heap.push(4);
        assert_eq!(&vec![0, 5, 4, 3, 2], heap.inner());
    }
}