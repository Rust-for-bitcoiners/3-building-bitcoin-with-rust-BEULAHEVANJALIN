#![allow(unused)]

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqAccess};
use std::fmt;
use std::marker::PhantomData;
use std::iter::Iterator;


/* This module will be taught in the class */

pub(crate) struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut acc = 0;
        let mut cur = &self.head;
        while let Some(ref node) = cur {
            acc += 1;
            cur = &node.next;
        }
        acc
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            val: value,
            next: self.head.take(),
        });
        self.head = Some(new_node); 
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next; 
            node.val
        })
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { val: value, next: None });
        let mut cur = &mut self.head;
        while let Some(ref mut next) = cur {
            cur = &mut next.next;
        }
        *cur = Some(new_node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut cur = &mut self.head;
        while cur.as_ref()?.next.is_some() {
            cur = &mut cur.as_mut()?.next;
        }
        cur.take().map(|node| node.val)
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref(); 
            &node.val
        })
    }
}

impl<T: Serialize> Serialize for LinkedList<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self.iter() {
            serde::ser::SerializeSeq::serialize_element(&mut seq, element)?;
        }
        serde::ser::SerializeSeq::end(seq)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for LinkedList<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinkedListVisitor<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T: Deserialize<'de>> Visitor<'de> for LinkedListVisitor<T> {
            type Value = LinkedList<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence representing a LinkedList")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut list = LinkedList::new();
                while let Some(value) = seq.next_element()? {
                    list.push_back(value);
                }
                Ok(list)
            }
        }

        deserializer.deserialize_seq(LinkedListVisitor {
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn push_front_adds_elements_correctly() {
        let mut list = LinkedList::new();
        list.push_front(1);
        assert_eq!(list.len(), 1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_front_removes_elements_correctly() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn push_back_adds_elements_correctly() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.len(), 1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_back_removes_elements_correctly() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn list_length_is_accurate() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_iteration() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(*iter.next().unwrap(), 2);
        assert_eq!(*iter.next().unwrap(), 3);
        assert!(iter.next().is_none());
    }
}