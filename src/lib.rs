#![allow(unused_variables)]
#![allow(dead_code)]
use static_rc::StaticRc;
use std::cell::RefCell;
use std::fmt;

type FullLink<T> = StaticRc<RefCell<Elt<T>>, 2, 2>;
type HalfLink<T> = StaticRc<RefCell<Elt<T>>, 1, 2>;

/// A dequeue is an ordered set of items supporting fast insertion and removal at both ends.
pub struct Dequeue<T>(Option<List<T>>);

/// A List is an always-nonempty doubly-linked list of items
struct List<T> {
    // head and tail are Option so that we can temporarily `.take()` them
    // in order to use the references during modification.  Outside of any
    // calls to methods on this struct, these are always `Some(_)`.
    head: Option<HalfLink<T>>,
    tail: Option<HalfLink<T>>,
}

/// An element in a List
struct Elt<T> {
    item: T,
    prev: Option<HalfLink<T>>,
    next: Option<HalfLink<T>>,
}

impl<T: fmt::Debug> Dequeue<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn insert_head(&mut self, item: T) {
        match &mut self.0 {
            None => self.0 = Some(List::new(item)),
            Some(ref mut list) => list.insert_head(item),
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        match &mut self.0 {
            None => None,
            Some(ref mut list) => {
                if let Some(item) = list.pop_head() {
                    Some(item)
                } else {
                    Some(self.0.take().unwrap().into_last_item())
                }
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Dequeue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Dequeue").field(&self.0).finish()
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("List")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .finish()
    }
}

impl<T: fmt::Debug> fmt::Debug for Elt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Elt")
            .field("item", &self.item)
            .field("prev", &self.prev.as_ref().map(|p| p.as_ptr()))
            .field("next", &self.next.as_ref().map(|p| p.as_ptr()))
            .finish()
    }
}

impl<T: fmt::Debug> List<T> {
    /// Create a new List containing the given item
    fn new(item: T) -> Self {
        let elt = Elt {
            item,
            prev: None,
            next: None,
        };
        let halves: (HalfLink<T>, HalfLink<T>) = StaticRc::split(StaticRc::new(RefCell::new(elt)));
        List {
            head: Some(halves.0),
            tail: Some(halves.1),
        }
    }

    /// Pop the head element, but if this would leave the list empty then return None
    fn pop_head(&mut self) -> Option<T> {
        let head_link = self.head.take().expect("List.head should be Some(_)");

        //debug_assert!(head_elt.borrow().next.is_none());

        // if there's a previous element to the head element, then we can disconnect the head
        // element and return it; otherwise, the caller must use [`into_last_item`]
        if let Some(prev_link) = StaticRc::get_ref(&head_link).borrow_mut().prev.take() {
            let prev_elt = StaticRc::get_ref(&prev_link);
            debug_assert!(!prev_elt.borrow().next.is_none());
            let prev_next_link = prev_elt
                .borrow_mut()
                .next
                .take()
                .expect("expected next to match next->prev");

            // link head to the new element
            self.head = Some(prev_link);

            // join both links to the old head so that we fully own the ref
            let full: FullLink<T> = StaticRc::join(prev_next_link, head_link);

            // using the fully-owned StaticRc, get the RefCell, and then get the contents of that
            // RefCell.
            Some(StaticRc::into_inner(full).into_inner().item)
        } else {
            self.head = Some(head_link);
            None
        }
    }

    fn into_last_item(mut self) -> T {
        let head_link = self.head.take().expect("List.head should be Some(_)");
        let tail_link = self.tail.take().expect("List.tail should be Some(_)");
        debug_assert!(StaticRc::get_ref(&head_link).borrow().prev.is_none());
        debug_assert!(StaticRc::get_ref(&tail_link).borrow().next.is_none());
        debug_assert!(StaticRc::ptr_eq(&head_link, &tail_link));

        // we have two references to the same object, so join them to take ownership,
        // and return the inner item as with pop_head
        let full: FullLink<T> = StaticRc::join(head_link, tail_link);
        StaticRc::into_inner(full).into_inner().item
    }

    fn insert_head(&mut self, item: T) {
        // before:
        //   self.head --> HEAD
        //   HEAD.next = None
        //   HEAD.prev = ??
        //
        // after:
        //   self.head --> NEW [DONE]
        //   NEW.prev --> HEAD [DONE]
        //   HEAD.next = Some(--> NEW) [DONE]

        let head_link = self.head.take().expect("List.head should be Some(_)");
        let elt = Elt {
            item,
            prev: Some(head_link),
            next: None,
        };
        let halves: (HalfLink<T>, HalfLink<T>) = StaticRc::split(StaticRc::new(RefCell::new(elt)));

        todo!()
        /*
        // elt is now owned by the half-links, but we need to set its
        let prev_link = StaticRc::get_ref(&halves.0).borrow().prev.as_ref().unwrap();
        StaticRc::get_ref(prev_link).borrow_mut().next = Some(halves.0);

        self.head = Some(halves.1);
            */
    }
}

#[test]
fn push_one_head_and_pop() {
    let mut d: Dequeue<u32> = Dequeue::new();
    d.insert_head(13);
    assert_eq!(d.pop_head(), Some(13));
    assert_eq!(d.pop_head(), None);
}
