///链表 栈实现

/// 链表节点元素
/// 
/// T为泛型类型，节点元素中装的T类型的数据
/// 
/// elem:数据域
/// 
/// next:指针域，指向下一个链节点
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// 链枚举
/// 
/// Empty：空链
/// 
/// More：链数据
pub enum Link<T> {
    /// 空节点枚举值
    Empty,
    /// Box智能指针将Node放在堆空间
    More(Box<Node<T>>),
}

/// 链表结构体
pub struct List<T> {
    /// 头结点
    head: Link<T>,
}

// 实现定义

impl<T> Drop for List<T> {

    /// List 结构 Drop特征实现
    fn drop(&mut self) {
        // 将头结点置为Empty，并返回头结点的值作为当前节点
        // cur_link退出drop作用域后会被自动drop
        let mut cur_link: Link<T> = std::mem::replace(&mut self.head, Link::Empty);

        // while循环，匹配模式，直到匹配到None,退出循环
        while let Link::More(mut box_node) = cur_link {
            // 将box_node置为Empty，并返回box_node.next作为当前节点
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
            cur_link = std::mem::replace(&mut box_node.next, Link::Empty);
        }
    }
}

// List结构体的实现
impl<T> List<T> {
    /// new函数， 返回List实例
    pub fn new() -> Self {
        // 构建个头结点位空的List实例，并返回
        List { head: Link::Empty }
    }

    /// 这个push是头插实现
    /// 
    /// 执行过程如下：
    ///
    /// 原：node2 -> node1
    ///
    /// 新建：node3
    ///
    /// 插入后：node3 -> node2 -> node3
    ///
    /// 最难理解的是：next: std::mem::replace(&mut self.head, Link::Empty)
    ///
    /// 将&mut self.head的值设置为Link::Empty，并返回&mut self.head的原值
    ///
    /// 原&mut self.head的值就是node2的地址
    ///
    /// 返回node2的地址，将node3的next指向node2
    ///
    /// 将&mut self.head的值置Link::Empty
    ///
    /// 最后将node3置为head
    pub fn push(&mut self, elem: T) {
        // 新建一个节点
        let new_node: Box<Node<T>> = Box::new(Node {
            elem,
            // 将&mut self.head的值设置为Link::Empty，并返回&mut self.head的原值
            // 没有交换之前，&mut self.head
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        // 将新节点插到头结点
        self.head = Link::More(new_node);
    }

    /// pop 函数
    pub fn pop(&mut self) -> Option<T> {
        // 定义result
        let result: Option<T>;
        // 将&mut self.head的值设置为Link::Empty，并返回&mut self.head的原值
        // 将head节点返回，并将head节点置空
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                // 将node.next置为head
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        // 返回值
        result
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
