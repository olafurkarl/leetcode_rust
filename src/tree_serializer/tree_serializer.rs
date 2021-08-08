use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

impl ToString for TreeNode {
    fn to_string(&self) -> String {
        format!("{}", self.val)
    }
}

pub struct TreeSerializer {
}

impl TreeSerializer {
    pub fn new() -> Self {
        Self { }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut vec_deque : VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        let mut result_string = String::from("");

        vec_deque.push_back(root);
        let mut current_level = 0;
        let mut next_level_has_some = true;
        while !vec_deque.is_empty() && next_level_has_some {
            let mut level_size = vec_deque.len();

            next_level_has_some = false;
            while level_size > 0 {
                level_size -= 1;
                if current_level != 0 {
                    result_string.push_str(",");
                }
                
                let r = vec_deque.pop_back().unwrap();
                match r {
                    Some(x) => {
                        let x = x.borrow();
                        result_string.push_str(&x.to_string());
                        vec_deque.push_front(x.left.clone());
                        vec_deque.push_front(x.right.clone());

                        if x.left.is_some() || x.right.is_some() { 
                            next_level_has_some = true;
                        }
                    },
                    None => {
                        result_string.push_str("null");
                    }
                }
            }
            current_level += 1;
        }

        return format!("[{}]", result_string);
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        // remove square brackets
        let data: &str = &data[1..data.len() - 1];

        let mut values: VecDeque<Option<i32>> = data.split(",").map(|x| {x.parse::<i32>().ok()}).collect();

        let mut need_children: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let root_val = values.pop_front().unwrap();

        let mut root_node: Option<Rc<RefCell<TreeNode>>> = None;

        if let Some(rv) = root_val {
            let node = TreeNode::new(rv);
            let node = Rc::new(RefCell::new(node));
            root_node = Some(Rc::clone(&node));
            need_children.push_front(Some(Rc::clone(&node)));
        }

        while !need_children.is_empty() && !values.is_empty() {
            let node = need_children.pop_front().unwrap().unwrap();
            let value = values.pop_front().unwrap();
            if let Some(v) = value {
                let child = TreeNode::new(v);
                let child = Rc::new(RefCell::new(child));
                need_children.push_back(Some(Rc::clone(&child)));
                node.borrow_mut().left = Some(Rc::clone(&child));
            }

            let value = values.pop_front().unwrap();
            if let Some(v) = value {
                let child = TreeNode::new(v);
                let child = Rc::new(RefCell::new(child));
                need_children.push_back(Some(Rc::clone(&child)));
                node.borrow_mut().right = Some(Rc::clone(&child));
            }
        }

        return root_node;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let obj = TreeSerializer::new();
        let data = "[1,2,3,null,null,4,5]".to_string();
        let root: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);

        assert!(root.is_some());

        fn get_children(node: Rc<RefCell<TreeNode>>) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
            return (node.borrow().left.clone(), node.borrow().right.clone())
        }

        fn assert_node(node: Option<Rc<RefCell<TreeNode>>>, val: i32) {
            let cloned = node.unwrap().borrow().val;
            assert_eq!(val, cloned);
        }

        let root = root.unwrap().clone();

        assert_eq!(1, root.borrow().val);

        let (child_left, child_right) = get_children(root);
                
        assert_node(child_left.clone(), 2);
        assert_node(child_right.clone(), 3);

        let (child_left2, child_right2) = get_children(child_left.unwrap());

        assert_eq!(child_left2, None);
        assert_eq!(child_right2, None);

        let (child_left3, child_right3) = get_children(child_right.unwrap());
        
        assert_node(child_left3.clone(), 4);
        assert_node(child_right3.clone(), 5);
    }

    #[test]
    fn test_serialize() {
        fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
            let node = TreeNode::new(val);
            let node = Rc::new(RefCell::new(node));
            node
        }

        let root = create_node(1);
        {
            let mut root_mut = root.borrow_mut();

            let left1 = create_node(2);
            let right1 = create_node(3);
    
            root_mut.left = Some(Rc::clone(&left1));
            root_mut.right = Some(Rc::clone(&right1));
    
            let mut right1 = right1.borrow_mut();
    
            let left3 = create_node(4);
            let right3 = create_node(5);
    
            right1.left = Some(Rc::clone(&left3));
            right1.right = Some(Rc::clone(&right3));
        }


        let obj = TreeSerializer::new();
        let expected = "[1,2,3,null,null,4,5]".to_string();

        let result: String = obj.serialize(Some(root));

        assert_eq!(expected, result);
    }
}