#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None
        }
    }

    pub fn boxed(val: i32) -> Box<Self> {
        Box::new(ListNode::new(val))
    }

    pub fn len(&self) -> usize {
        if let Some(next) = &self.next {
            return next.len() + 1
        } else {
            1
        }
    }

    pub fn addon(&mut self, addon: ListNode) {
        if let Some(next) = self.next.as_mut() {
            next.addon(addon);
        } else {
            self.next = Some(Box::new(addon));
        }
    }

    // TODO: 垃圾，重写
    // 复用每个节点，仅修改其 next 值来反转其顺序
    // 最终返回的为最后的节点
    pub fn reverse(&mut self) -> ListNode {
        let n;
        if let Some(next) = self.next.as_mut() {
            n = ListNode::new(self.val);
            let mut t = next.reverse();
            t.addon(n);
            t
        } else {
            ListNode::new(self.val)
        }
    }

    // 按节点序生成 Vector
    pub fn to_vec(self) -> Vec<ListNode> {
        let mut res = Vec::new();

        fn _to_vec(mut ln: ListNode, v: &mut Vec::<ListNode>) {
            if let Some(next) = ln.next {
                let next = *next;
                ln.next = None;
                v.push(ln);
                _to_vec(next, v);
            } else {
                v.push(ln);
            }
        }

        _to_vec(self, &mut res);
        res
    }

    // 按序连接各节点
    pub fn from_vec(v: &mut Vec<ListNode>) -> Option<Self> {
        let mut res = None;
        loop {
            if let Some(mut node) = v.pop() {
                node.next = res;
                res = Some(Box::new(node));
            } else {
                return Some(*res.unwrap())
            }
        }
    }

}

pub fn append(mut head: ListNode, body: ListNode) -> ListNode {
    head.next = Some(Box::new(body));
    head
}

mod test {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(ListNode {
                       val: 0,
                       next: None
                   },
                   ListNode::new(0)

        );
    }

    #[test]
    fn test_to_vec() {
        let mut v = Vec::new();
        for i in 0..3 {
            v.push(ListNode::new(i));
        }

        let lns = ListNode {
            val: 0,
            next: Some(Box::new(
                ListNode {
                    val: 1,
                    next: Some(Box::new(
                        ListNode {
                            val: 2,
                            next: None
                        }
                    ))
                }
            ))
        };

        assert_eq!(lns.to_vec(), v);
    }

    #[test]
    fn test_from_vec() {
        let mut v = Vec::new();
        for i in 0..3 {
            v.push(ListNode::new(i));
        }
        let lns = ListNode::from_vec(&mut v).unwrap();
        assert_eq!(ListNode {
                       val: 0,
                       next: Some(Box::new(
                           ListNode {
                               val: 1,
                               next: Some(Box::new(ListNode::new(2))),
                           }
                       ))
                   },
                   lns
                );
    }

    #[test]
    fn test_len() {
        for i in 1..4 {
            let mut v = Vec::new();
            for j in 0..i {
                v.push(ListNode::new(j));
            }

            assert_eq!(ListNode::from_vec(&mut v).unwrap().len(), i as usize);
        }
    }
}

fn main() {
    println!("Hello World!");
    println!("Please run test!");
}