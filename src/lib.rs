#![allow(dead_code)]

pub mod problems;
pub mod utils;

// struct Solution;

#[macro_export]
macro_rules! batch_test {
    ($func:path: $(($($inputs:expr),*) => $res:expr),+$(,)?) => {
        $(assert_eq!($func($($inputs),*), $res);)+
    };
}

#[macro_export]
macro_rules! list {
    ($($elem:expr),*) => {
        {
            let _vec = vec![$($elem),*];

            let mut head = None;
            let mut tail = &mut head;

            for e in _vec {
                let node = ListNode::new(e);
                *tail = Some(Box::new(node));
                tail = &mut tail.as_mut().unwrap().next;
            }

            head
        }
    };
}
