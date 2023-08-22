// 为了链表可以被引用多次(单线程)，使用Rc计数器
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Rc不支持可变引用借用，不纠结了
    // pub fn iter_mut(&mut self) -> IterMut<T> {
    //     let mut next = None;
    //     if let Some(ref )
    // }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            // as_deref 将智能指针类型转换为&引用类型
            next: self.head.as_deref(),
        }
    }

    /// new 关联方法
    pub fn new() -> Self {
        List { head: None }
    }

    /// 添加元素，并返回新链表
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                // 该处clone只是调用Rc的clone函数，链表引用+1
                next: self.head.clone(),
            })),
        }
    }

    /// 将现有链表的首个元素移除，并返回剩余的链表
    pub fn tail(&self) -> List<T> {
        List {
            // head的类型为 Option<Rc<Node<T>>>
            head: self
                .head
                // as_ref:Option<Rc<Node<T>>> 类型转换为 Option<&Rc<Node<T>>> 类型
                .as_ref()
                // and_then: 如果选项为 None，则返回 None; 否则，使用包装的值调用 f，并返回结果
                .and_then(|node: &Rc<Node<T>>| node.next.clone()),
        }
    }

    /// 返回首个元素的引用
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// 链表的Drop实现
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link: Option<Rc<Node<T>>> = self.head.take();
        while let Some(cur_node) = cur_link {
            if let Ok(mut cur_node_0) = Rc::try_unwrap(cur_node) {
                cur_link = cur_node_0.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Iter, List};

    #[test]
    pub fn test() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list: List<i32> = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list: List<i32> = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list: List<i32> = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list: List<i32> = list.tail();
        assert_eq!(list.head(), None);

        let list: List<i32> = list.tail();
        assert_eq!(list.head(), None);

        let list: List<i32> = list.prepend(1).prepend(2).prepend(3);
        let mut iter: Iter<'_, i32> = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
