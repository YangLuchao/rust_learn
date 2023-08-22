use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/// 双向链表结构
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

/// 链表节点包装
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// 链表节点
pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

/// 转换为迭代器
pub struct IntoIter<T>(List<T>);

/// 转换为迭代器引用
pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

/// 节点实现
impl<T> Node<T> {
    /// 节点new函数
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

/// 链表实现
impl<T> List<T> {

    pub fn new() -> List<T>{
        List { head: None, tail: None }
    }

    /// 转换为迭代器引用实现
    pub fn iter(&self) -> Iter<T>{
        // Iter 包装的类型为：Option<Ref<'a, Node<T>>>
        Iter(self.head.as_ref().map(|head: &Rc<RefCell<Node<T>>>| head.borrow()))
    }

    /// 转换为迭代器实现
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// 从头插入节点
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            // 如果头结点不为空
            // 1:将头结点设置为None
            // 2:将新建的节点置为头结点
            // 3:将新头节点的next指向老节点
            // 4:将老节点的prev置为新节点
            Some(old_head) => {
                // 非空链表，需要将新的节点和老的头部连接
                // old_head的类型为 Rc<RefCell<Node<T>>> ，先调用RefCell的borrow_mut函数，将不可变引用转变为可变引用，才可以设置prev的值
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // 如果头节点为空
                // 1:设置新节点为头结点
                // 2:将新节点的引用复制一个给tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    /// 尾插
    pub fn push_back(&mut self, elem: T) {
        let new_tail: Rc<RefCell<Node<T>>> = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    /// 尾弹
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail: Rc<RefCell<Node<T>>>| {
            match old_tail.borrow_mut().next.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().prev.take();
                    self.tail = Some(new_tail);
                }, 
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
        
    }

    /// 从头弹出节点
    pub fn pop_front(&mut self) -> Option<T> {
        // 拿到头结点
        self.head.take().map(|old_head: Rc<RefCell<Node<T>>>| {
            // old_head为Rc<RefCell<Node<T>>>类型，通过borrow_mut()函数转为可变引用
            // old_head的next置为空，并返回 old_head的next 的值
            match old_head.borrow_mut().next.take() {
                // 有值，类型为 Rc<RefCell<Node<T>>>
                // 将 old_head的next 从命名为 new_head
                Some(new_head) => {
                    // new_head 的 prev 指针置为none
                    new_head.borrow_mut().prev.take();
                    // 将链表的头结点置为new_head
                    self.head = Some(new_head);
                }
                // 如果头结点没有next,就说明当前节点为最后一个节点
                // 所以讲尾指针置为None
                None => {
                    self.tail.take();
                }
            }
            // 返回old_head 里的值
            // Rc::try_unwrap(): 如果 old_head 只有一个强引用，则返回内部的值
            // 类型为 Result<RefCell<Node<T>>, Rc<RefCell<Node<T>>>>
            // ok()：将Result类型转换为 Options类型： Option<RefCell<Node<T>>>
            // unwrap(): 拿到Option内的值，类型为：RefCell<Node<T>>
            // into_inner(): 消耗RefCell 返回RefCell内包装的值：类型为Node<T>
            // elem: 最后拿到Node内包装的值T，并返回被包装成Option<T>
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        // self.head 的类型为 Option<Rc<RefCell<Node<T>>>>
        self.head
            // as_ref(): 将Option内的包装值转换为不可变引用 Option<&Rc<RefCell<Node<T>>>>
            .as_ref()
            // map():将包装内的值，类型为：&Rc<RefCell<Node<T>>> 映射为新值返回
            .map(|node: &Rc<RefCell<Node<T>>>| 
                // 返回类型为Ref，所以用Ref::map包裹，函数含义为:将原Ref<T> 转换为 Ref<U> 后返回
                // 原：node.borrow()的类型为：Ref<'_, Node<T>>
                // fn：转换闭包：node类型 &Node<T> 要转换为 &T -> &node.elem
                Ref::map(node.borrow(), |node: &Node<T>| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node: &Rc<RefCell<Node<T>>> | {
            Ref::map(node.borrow(), |node: &Node<T>| &node.elem)
        })
    }

    pub fn peek_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node: &Rc<RefCell<Node<T>>>| 
            RefMut::map(node.borrow_mut(), |node: &mut Node<T>| &mut node.elem)
        )
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node: &Rc<RefCell<Node<T>>>| 
            RefMut::map(node.borrow_mut(), |node: &mut Node<T>| &mut node.elem)
        )
    }

}

/// 转换迭代器实现，从前往后迭代
impl <T> Iterator for IntoIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

/// 从后往前迭代
impl <T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

/// 链表的Drop实现
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // self.pop_front().is_some() 判断是否有值
        // self.pop_front()返回的类型为Option<T>
        // 作用域为while循环体，完成一次循环体后，返回的Option<T>就会被自动Drop
        while self.pop_front().is_some() {}
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();
        assert!(list.peek().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek().unwrap(), &3);
        assert_eq!(&mut *list.peek_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

}