/// 链节点对象，Option别名
///
/// 智能指针Box将Node放在堆区
type Link<T> = Option<Box<Node<T>>>;

/// 转换为迭代器结构
///
/// 返回节点实例
pub struct IntoIter<T>(List<T>);

/// 迭代构造
///
/// 当持有当前节点的指针，当生成一个值后，该指针指向下一个节点
///
/// ’a生命周期意味着，Node<T>引用的生命周期要比Iter的长
///
/// 返回节点不可变引用
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// 迭代结构
///
/// 返回节点的可变引用
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

/// 数据节点结构，存储泛型类型
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// 链表结构体
pub struct List<T> {
    /// 头结点
    head: Link<T>,
}

/// 链表实现
impl<T> List<T> {
    /// 这里的生命周期'a， &self获得至少和Iter一样久
    ///
    /// 转换为Iter对象
    pub fn iter(&self) -> Iter<T> {
        /*
            as_ref() 和 as_mut() 用于返回内部值的引用，分别是不可变引用和可变引用。
            as_deref() 和 as_deref_mut() 则用于解引用操作，将包装类型转换为内部类型的引用。
            as_deref()转换为不可变引用， as_deref_mut()转换为可变引用
        */
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// 转换为IterMut对象
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    /// 链表转换为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // new 链表
    pub fn new() -> Self {
        List { head: None }
    }

    /// push方法
    pub fn push(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            elem,
            // take：将值取出后返回，原值置为None
            next: self.head.take(),
        });
        // 头结点设置为新建节点
        self.head = Some(new_node);
    }

    /// pop 方法
    pub fn pop(&mut self) -> Option<T> {
        // 返回头结点值，返回值为Option，进入match匹配模式
        // 返回头结点的值，并将头结点的值设置为None
        match self.head.take() {
            // 匹配到None，直接返回None
            None => None,
            // 匹配到Some
            Some(node) => {
                // 将头结点置为node.next
                self.head = node.next;
                // 将头结点的值返回
                Some(node.elem)
            }
        }
    }

    /// 返回头结点元素的不可变引用
    pub fn peek(&self) -> Option<&T> {
        // 如果不加as_ref，就会讲head节点中元素的所有权转移到map中
        // 加上as_ref，将head节点中的元素的引用转移到map中
        self.head.as_ref().map(|node: &Box<Node<T>>| &node.elem)
    }

    /// 返回头结点的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // self.head 的类型为 Option<Box<Node<T>>>
        self.head
            // 转换为 Option<&mut Box<Node<T>>>
            .as_mut()
            // 最后将节点中元素的可变引用返回
            .map(|node: &mut Box<Node<T>>| &mut node.elem)
    }
}

/// 链表实现Drop特征
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // 头结点返回作为当前节点，并将原头结点设置为None
        let mut cur_link: Option<Box<Node<T>>> = self.head.take();
        // while 循环 定义新变量cur_node ,cur_node = cur_link
        // 如果cur_node 为Some，则进入循环体
        // cur_node 作用域为while循环，完成一次循环体，则当前cur_node将被自动drop
        while let Some(mut cur_node) = cur_link {
            // cur_node.next返回赋值为cur_link，cur_node.next设置为None
            cur_link = cur_node.next.take();
        }
    }
}

/// 链表实例迭代器实现
impl<T> Iterator for IntoIter<T> {
    // 关联类型
    type Item = T;

    /// 迭代器默认函数，next
    fn next(&mut self) -> Option<Self::Item> {
        // 为什么是0，因为IntoIter是元组结构，0是元组结构的第一个元素
        // pop是已实现的弹出函数
        self.0.pop()
    }
}

/// 不可变引用迭代器实现
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.next 类型为 Option<&Node<T>>
        // map 函数是将 &Node<T> 类型的数据进行copy
        // 然后在闭包中处理
        self.next.map(|node: &Node<T>| {
            // as_deref 函数将 Option<Box<Node<T> 类型转换为 Option<&Node<T>> 类型
            // 就是将Option中的智能指针类型转换为不可变引用类型
            // self.next指向 node.next.as_deref()
            self.next = node.next.as_deref();
            // 将元素的不可变引用返回
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.next类型为Option<&mut Node<T>>
        // 可变引用 Option<&mut T> 没有实现copy特性，所以后面不能跟map讲可变引用的借走
        // take 方法将 Option 中的值取出并返回，就不会在编译期被执行copy，使得 map 方法可以应用于该值
        self.next.take().map(|node: &mut Node<T>| {
            // as_deref 函数将 Option<Box<Node<T> 类型转换为 Option<&mut Node<T>> 类型
            // 就是将Option中的智能指针类型转换为可变引用类型
            self.next = node.next.as_deref_mut();
            // 将元素的可变引用返回
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::{IntoIter, List};

    #[test]
    pub fn test1() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter: IntoIter<i32> = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value: &mut i32| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
