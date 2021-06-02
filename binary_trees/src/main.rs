
struct Node<T: PartialOrd> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: PartialOrd + Copy> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            left: None,
            right: None,

        }
    }
    fn insert(self: &mut Node<T>, val: T) {
        if self.val > val {
            match &mut self.left {
                Some(v) => v.insert(val),
                None => {
                    let new_leaf = Node::new(val);
                    self.left = Some(Box::new(new_leaf));
                }
    
            }
        } else if self.val < val {
            match &mut self.right {
                Some(v) => v.insert(val),
                None => {
                    let new_leaf = Node::new(val);
                    self.right = Some(Box::new(new_leaf));
                }
    
            }
        }
    }
    fn max(self: &Node<T>) -> T {
        match &self.right {
            Some(v) => v.max(),
            None => return self.val,
        }
    }
    
    fn second_max(self: &Node<T>) -> T {
        let carrier_val = self.val;
        self.second_max_helper(carrier_val)
    }
    fn second_max_helper(self: &Node<T>, carrier: T) -> T {
        match &self.right {
            Some(v) => v.second_max_helper(self.val),
            None => {
                match &self.left {
                    Some(w) => {
                        if w.val > carrier {
                            return w.val;
                        } else {
                            return carrier;
                        }
                    }
                    None => carrier
                }
            }
        }
    }    
}

fn main() {
    let mut my_float_bin_tree = Node::new(5.32);
    my_float_bin_tree.insert(10.1);
    my_float_bin_tree.insert(13.2);
    my_float_bin_tree.insert(1.0);
    my_float_bin_tree.insert(7.0);
    my_float_bin_tree.insert(15.5);
    my_float_bin_tree.insert(11.8);

    println!("Higest value in tree is: {}", &my_float_bin_tree.max());
    println!("Next highest: {}", &my_float_bin_tree.second_max());
}
