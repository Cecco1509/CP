pub struct Node {
    key: i32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: i32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> i32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> i32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Checks if the tree is a Binary Search Tree
    pub fn is_bts(&self) -> bool {
        self.rec_is_bts(Some(0), true).0
    }

    /// Checks if the given subtree is a Binary Search Tree and returns the maximum or minimum value in the subtree.
    /// 
    /// # Parameters
    /// * option - This boolean parameter specifies whether the returned key value should be 
    /// the minimum or maximum of the subtree.
    ///
    /// # Returns
    /// return values ( is_bst : bool, value_requested: Option<i32> )
    /// value_requested refers to the value specified in parameter options
    fn rec_is_bts(&self, node_id: Option<usize>, option: bool) -> (bool, i32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let mut ret_key: i32 = node.key; // Stores the return key value

            // Checks the value of the left child node 
            if let Some(id_left) = node.id_left {
                if node.key < self.nodes[id_left].key {
                    return (false, -1);
                }

                let left: (bool, i32) = self.rec_is_bts(node.id_left, true);

                if !left.0 { return (false, -1); }

                if left.1 > node.key { return (false, -1); }

                if !option { ret_key = left.1; }
            }

            if let Some(id_right) = node.id_right {
                if node.key > self.nodes[id_right].key {
                    return (false, -1);
                }

                let right: (bool, i32) = self.rec_is_bts(node.id_right, false);

                if !right.0 { return (false, -1); }

                if right.1 < node.key  { return (false, -1); }
                
                if option { ret_key = right.1 }
            }

            return (true, ret_key);
        }

        (false, -1)
    }


    /// Returns the maximum path sum between two leaves in the tree.
    /// If no path is found then it returns None.
    pub fn max_path_sum(&self) -> Option<i32> {
        self.rec_max_path_sum(Some(0)).1
    }

    /// Returns a couple containing the max path sum to a leaf 
    /// and the max path sum between two leaves of the given node.
    ///
    /// Returned values could be None so it's used Option<i32>
    fn rec_max_path_sum(&self, node_id: Option<usize>) -> (Option<i32>, Option<i32>) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            // Evaluates the maximum path in the left and right subtrees
            let left = self.rec_max_path_sum(node.id_left);
            let right = self.rec_max_path_sum(node.id_right);

            let mut max_sum_child: Option<i32> = None; // Stores the max path sum retrieved form the child nodes
            let mut result: (i32, Option<i32>) = (node.key, None); // Stores the values that will be returned

            // This checks whether or not the node is a leaf.
            // If it is then returns only the node key value as the max path sum to a leaf.
            if left.0.is_none() && right.0.is_none() {
                return (Some(node.key), None);
            }

            // This compares the max path sum retreived from the left and right nodes.
            if left.1.is_some() && right.1.is_some() {
                if left.1.unwrap() > right.1.unwrap() {
                    max_sum_child = left.1;
                } else {
                    max_sum_child = right.1;
                }
            } else if left.1.is_some() {
                max_sum_child = left.1;
            } else if right.1.is_some() {
                max_sum_child = right.1;
            }

            // If there is a path to a leaf from both the left and right child nodes
            // then it adds the greater one to the result
            if left.0.is_some() && right.0.is_some() {
                if left.0.unwrap() > right.0.unwrap() {
                    result.0 += left.0.unwrap();
                } else {
                    result.0 += right.0.unwrap();
                }

                // this checks if the max sum path evaluated before is greater
                // than the path that it can be created combining this node and its children
                if max_sum_child.is_some() {
                    if left.0.unwrap() + node.key + right.0.unwrap() > max_sum_child.unwrap() {
                        result.1 = Some(left.0.unwrap() + node.key + right.0.unwrap())
                    } else {
                        result.1 = max_sum_child;
                    }
                } else {
                    result.1 = Some(left.0.unwrap() + node.key + right.0.unwrap())
                }
            } else if left.0.is_some() {
                // this runs if the right child doesn't exist
                result.0 += left.0.unwrap();
                result.1 = max_sum_child;
            } else if right.0.is_some() {
                // this runs if the left child doesn't exist
                result.0 += right.0.unwrap();
                result.1 = max_sum_child;
            }

            // returns the evaluated results
            return (Some(result.0), result.1);
        }

        (None, None)
    }

    //if (left.0.is_none() && left.1.is_none()) && (right.0.is_none() && right.1.is_none()) { return (Some(node.key), None);}
}

#[cfg(test)]
mod is_bst_tests {
    use super::*;

    #[test]
    fn valid_bts() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bts(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.is_bts(), true);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.is_bts(), true);
    }

    #[test]
    fn not_valid_bts() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 11, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.is_bts(), false);

        let mut tree1 = Tree::with_root(10);

        tree1.add_node(0, 9, true); // id 1
        tree1.add_node(0, 9, false); // id 2

        assert_eq!(tree1.is_bts(), false);
    }

    #[test]
    fn single_node_bts() {
        let tree = Tree::with_root(10);

        assert_eq!(tree.is_bts(), true);
    }

    #[test]
    fn only_left_nodes_bts() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 9, true); // id 1
        tree.add_node(1, 7, true); // id 3

        assert_eq!(tree.is_bts(), true);
    }

    #[test]
    fn only_right_nodes_bts() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 11, false); // id 1
        tree.add_node(1, 12, false); // id 3

        assert_eq!(tree.is_bts(), true);
    }

    #[test]
    fn nested_tree_bts_1() {
        let mut tree = Tree::with_root(20);

        //       20
        //     /    \
        //   10      21
        //  /  \
        // 8   15
        tree.add_node(0, 10, true); // id 1
        tree.add_node(0, 21, false); // id 2
        tree.add_node(1, 8, true); // id 3
        tree.add_node(1, 15, false); // id 4

        assert_eq!(tree.is_bts(), true);

        let mut tree1 = Tree::with_root(20);

        //       20
        //     /    \
        //   10      21
        //  /  \
        // 8   22
        tree1.add_node(0, 10, true); // id 1
        tree1.add_node(0, 21, false); // id 2
        tree1.add_node(1, 8, true); // id 3
        tree1.add_node(1, 22, false); // id 4

        assert_eq!(tree1.is_bts(), false);
    }

    #[test]
    fn nested_tree_bts_2() {
        let mut tree = Tree::with_root(20);

        //       20
        //     /    \
        //   10     30
        //         /  \
        //        28   31
        tree.add_node(0, 10, true); // id 1
        tree.add_node(0, 30, false); // id 2
        tree.add_node(2, 28, true); // id 3
        tree.add_node(2, 31, false); // id 4

        assert_eq!(tree.is_bts(), true);

        let mut tree1 = Tree::with_root(20);

        //       20
        //     /    \
        //   10     30
        //         /  \
        //        19   31
        tree1.add_node(0, 10, true); // id 1
        tree1.add_node(0, 30, false); // id 2
        tree1.add_node(2, 19, true); // id 3
        tree1.add_node(2, 31, false); // id 4

        assert_eq!(tree1.is_bts(), false);
    }

    #[test]
    fn not_bts_subtree() {
        let mut tree = Tree::with_root(20);

        //       20
        //     /    \
        //   10      21
        //  /  \
        // 16   15
        tree.add_node(0, 10, true); // id 1
        tree.add_node(0, 21, false); // id 2
        tree.add_node(1, 16, true); // id 3
        tree.add_node(1, 15, false); // id 4

        assert_eq!(tree.is_bts(), false);

        let mut tree1 = Tree::with_root(20);

        //       20
        //     /    \
        //   10     30
        //         /  \
        //        40   31
        tree1.add_node(0, 10, true); // id 1
        tree1.add_node(0, 30, false); // id 2
        tree1.add_node(2, 19, true); // id 3
        tree1.add_node(2, 31, false); // id 4

        assert_eq!(tree1.is_bts(), false);
    }
}

#[cfg(test)]
mod max_path_tests {
    use super::*;

    #[test]
    fn simple_tree() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 5, false); // id 3

        assert_eq!(tree.max_path_sum(), Some(20));
    }

    #[test]
    fn right_subtree() {
        let mut tree = Tree::with_root(20);

        //       20
        //     /    \
        //   10      9
        //         /  \
        //        10   5
        tree.add_node(0, 10, true); // id 1
        tree.add_node(0, 9, false); // id 2
        tree.add_node(2, 10, true); // id 3
        tree.add_node(2, 5, false); // id 4

        assert_eq!(tree.max_path_sum(), Some(49));
    }

    #[test]
    fn negative_values() {
        let mut tree = Tree::with_root(-20);

        //       -20
        //     /    \
        //   -10     9
        //         /  \
        //        10   5
        tree.add_node(0, -10, true); // id 1
        tree.add_node(0, 9, false); // id 2
        tree.add_node(2, 10, true); // id 3
        tree.add_node(2, 5, false); // id 4

        assert_eq!(tree.max_path_sum(), Some(24));
    }

    #[test]
    fn more_negative_values() {
        let mut tree = Tree::with_root(-20);

        //       -20
        //     /    \
        //   100   -10
        //         /  \
        //       20   -5
        //              \
        //              50
        tree.add_node(0, 100, true); // id 1
        tree.add_node(0, -10, false); // id 2
        tree.add_node(2, 20, true); // id 3
        tree.add_node(2, -5, false); // id 4
        tree.add_node(4, 50, false); // id 5

        assert_eq!(tree.max_path_sum(), Some(115));

        let mut tree = Tree::with_root(-20);

        //       -20
        //     /    \
        //   100   -10
        //         /  \
        //       20   -5
        //           /  \
        //          -5  50
        //           \
        //           56
        tree.add_node(0, 100, true); // id 1
        tree.add_node(0, -10, false); // id 2
        tree.add_node(2, 20, true); // id 3
        tree.add_node(2, -5, false); // id 4
        tree.add_node(4, 50, false); // id 5
        tree.add_node(4, -5, true); // id 6
        tree.add_node(6, 56, false); // id 7

        assert_eq!(tree.max_path_sum(), Some(116));
    }

    #[test]
    fn absent_path() {
        let tree = Tree::with_root(10);

        //     10
        //
        assert_eq!(tree.max_path_sum(), None);

        let mut tree = Tree::with_root(10);

        //     10
        //    /
        //   5
        tree.add_node(0, 5, true); // id 1

        assert_eq!(tree.max_path_sum(), None);

        let mut tree = Tree::with_root(10);

        //     10
        //       \
        //        5
        tree.add_node(0, 5, false); // id 1

        assert_eq!(tree.max_path_sum(), None);
    }

    #[test]
    fn test6() {
        let mut tree = Tree::with_root(20);

        //         20
        //       /   \
        //    -10    10
        //    /     /  \
        //    0    35  50
        //  /  \
        // 100 100
        tree.add_node(0, -10, true); // id 1
        tree.add_node(0, 10, false); // id 2
        tree.add_node(1, 0, true); // id 3
        tree.add_node(2, 35, true); // id 4
        tree.add_node(2, 50, false); // id 5
        tree.add_node(3, 100, true); // id 6
        tree.add_node(3, 100, false); // id 7

        assert_eq!(tree.max_path_sum(), Some(200));
    }

    #[test]
    fn test7() {
        let mut tree = Tree::with_root(-15);

        //            -15
        //          /      \
        //          5         6
        //        /  \       / \
        //      -8    1     3   9
        //     /  \              \
        //    2   -3              0
        //                       / \
        //                      4  -1
        //                         /
        //                       10
        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 6, false); // id 2
        tree.add_node(1, -8, true); // id 3
        tree.add_node(1, 1, false); // id 4
        tree.add_node(2, 3, true); // id 5
        tree.add_node(2, 9, false); // id 6
        tree.add_node(3, 2, true); // id 7
        tree.add_node(3, -3, false); // id 8
        tree.add_node(6, 0, false); // id 9
        tree.add_node(9, 4, true); // id 10
        tree.add_node(9, -1, false); // id 11
        tree.add_node(11, 10, true); // id 12

        assert_eq!(tree.max_path_sum(), Some(27))
    }

    #[test]
    fn test8() {
        let mut tree = Tree::with_root(3);

        //            3
        //         /    \
        //       4       5
        //      /  \
        //    -10   4
        tree.add_node(0, 4, true);
        tree.add_node(0, 5, false);
        tree.add_node(1, -10, true);
        tree.add_node(1, 4, false);

        assert_eq!(tree.max_path_sum(), Some(16))
    }
}
