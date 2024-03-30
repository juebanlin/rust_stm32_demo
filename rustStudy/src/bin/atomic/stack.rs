#![feature(box_raw)]

use std::ptr::{self, null_mut};
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release,AcqRel,SeqCst};

pub struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: AtomicPtr::new(null_mut()),
        }
    }
    pub fn pop(&self) -> Option<T> {
        loop {
            // 快照
            let head = self.head.load(Acquire);
            // 如果栈为空
            if head == null_mut() {
                return None;
            } else {
                let next = unsafe { (*head).next };
                // 如果现状较快照并没有发生改变
                if self.head.compare_and_swap(head, next, Release) == head {
                    // 读取内容并返回
                    return Some(unsafe { ptr::read(&(*head).data) });
                }
            }
        }
    }

    pub fn push(&self, t: T) {
        // 创建node并转化为*mut指针
        let n = Box::into_raw(Box::new(Node {
            data: t,
            next: null_mut(),
        }));
        loop {
            // 快照
            let head = self.head.load(Relaxed);
            // 基于快照更新node
            unsafe {
                (*n).next = head;
            }
            // 如果在此期间，快照仍然没有过时
            if self.head.compare_and_swap(head, n, Release) == head {
                break;
            }
        }
    }
}

fn main() {

}