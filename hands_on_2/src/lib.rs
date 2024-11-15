use std::cmp::{max, min};

pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub struct RangeNode {
    key: i32,
    range: Range,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl RangeNode {
    fn new(key: i32, range: Range) -> Self {
        Self {
            key,
            range,
            id_left: None,
            id_right: None,
        }
    }

    pub fn clone_node(&self) -> RangeNode {
        RangeNode::new(self.key, Range::new(self.range.start, self.range.end))
    }
}

pub struct LevelNode {
    range_node: RangeNode,
    level: usize,
}

impl LevelNode {
    fn new(range_node: RangeNode, l: usize) -> Self {
        Self {
            range_node: range_node.clone_node(),
            level: l,
        }
    }

    pub fn clone_node(&self) -> LevelNode {
        LevelNode::new(self.range_node.clone_node(), self.level)
    }
}

pub struct MinMax {
    nodes: Vec<RangeNode>,
    lazy_nodes: Vec<Option<i32>>,
}

fn tree_init(arr: &Vec<i32>, start: usize, end: usize, level: usize) -> (Vec<LevelNode>, i32) {
    if start == end {
        return (
            vec![LevelNode::new(
                RangeNode::new(arr[start], Range::new(start, end)),
                level,
            )],
            arr[start],
        );
    }

    let mid = (start + end) / 2;
    let (left_seg_tree, left_max) = tree_init(arr, start, mid, level + 1);
    let (right_seg_tree, right_max) = tree_init(arr, mid + 1, end, level + 1);
    let maximum = max(left_max, right_max);

    let mut ret_arr: Vec<LevelNode> = Vec::with_capacity((2 * left_seg_tree.len()) + 1);
    ret_arr.push(LevelNode::new(
        RangeNode::new(maximum, Range::new(start, end)),
        level,
    ));
    merge(&left_seg_tree, &right_seg_tree, &mut ret_arr);

    (ret_arr, maximum)
}

fn merge(left: &[LevelNode], right: &[LevelNode], merged_arr: &mut Vec<LevelNode>) {
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i].level <= right[j].level {
            merged_arr.push(left[i].clone_node());
            i += 1;
        } else {
            merged_arr.push(right[j].clone_node());
            j += 1;
        }
    }

    while i < left.len() {
        merged_arr.push(left[i].clone_node());
        i += 1;
    }

    while j < right.len() {
        merged_arr.push(right[j].clone_node());
        j += 1;
    }
}

impl MinMax {

    /* Complexity = O(n log n) used merge sort to build the tree merge sort*/
    pub fn new(arr: Vec<i32>) -> Self {
        let (ranges, _) = tree_init(&arr, 0, arr.len()-1, 0);
        let mut lazy_tree: Vec<Option<i32>> = Vec::with_capacity(ranges.len());
        let mut seg_tree: Vec<RangeNode> = Vec::with_capacity(ranges.len());

        let mut nonode = 0;

        for (i, node) in ranges.iter().enumerate() {
            let mut new_node = node.range_node.clone_node();

            if new_node.range.end - new_node.range.start == 0 {
                new_node.id_left = None;
                new_node.id_right = None;
                nonode += 1;
            } else {
                new_node.id_left =  Some(((i - nonode) * 2) + 1);
                new_node.id_right = Some(((i - nonode) * 2) + 2);
            }

            seg_tree.push(new_node);
            lazy_tree.push(None);
        }

        Self {
            nodes: seg_tree,
            lazy_nodes: lazy_tree,
        }
    }

    /* complexity of one query O(log n), total queries m -> O(m * log n) total complexity O((n + m) * log n) */ 
    pub fn query(&mut self, query: usize, start: usize, end: usize, t: i32) -> Option<i32> {
        if query == 0 {
            self.update(start - 1, end - 1, t, Some(0));
        }
        self.max(start - 1, end - 1, Some(0))
    }

    fn max(&mut self, start: usize, end: usize, node: Option<usize>) -> Option<i32> {
        if let Some(node) = node {
            
            self.update_node(node);
            
            if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
                return None;
            }

            if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {
                return Some(self.nodes[node].key);
            }

            if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {

                let max_left = self.max(start, end, self.nodes[node].id_left);
                let max_right = self.max(start, end, self.nodes[node].id_right);

                if max_left.is_none() && max_right.is_none() {
                    return Some(self.nodes[node].key);
                }

                if max_left.is_none() {
                    return max_right;
                }

                if max_right.is_none() {
                    return max_left;
                }

                return std::cmp::max(max_left, max_right);
            }
        }

        None
    }

    fn update_node(&mut self, node: usize) {

        if let Some(new_val) = self.lazy_nodes[node] {

            if new_val <= self.nodes[node].key {
                self.nodes[node].key = new_val;
                //println!("NODE UPDATED : {:?} to node {} range {}-{}", self.nodes[node].key, node, self.nodes[node].range.start, self.nodes[node].range.end);

                self.propagate(node, new_val);

                self.lazy_nodes[node] = None;
            }

        }

    }

    fn propagate(&mut self, node: usize, t:i32) {
        if self.nodes[node].id_left.is_none() { return; }

        let left_id = self.nodes[node].id_left.unwrap();
        let right_id = self.nodes[node].id_right.unwrap();

        self.lazy_nodes[left_id] = Some(t);
        self.lazy_nodes[right_id] = Some(t);
    }

    fn update(&mut self, start: usize, end: usize, t: i32, node: Option<usize>) -> Option<i32> {
        if let Some(node) = node {

            self.update_node(node);

            // Nessuna sovrapposizione
            if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
                return Some(self.nodes[node].key);
            }

            // Sovrapposizione completa
            if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {

                if self.nodes[node].key > t {
                    self.nodes[node].key = t;
                    //println!("ASSIGNED NODE : {} to node {} range {}-{}", t, node, self.nodes[node].range.start, self.nodes[node].range.end);
                    self.propagate(node, t);
                }

                return Some(self.nodes[node].key);
            }

            // Sovrapposizione parziale
            if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {

                let left_id = self.nodes[node].id_left;
                let right_id = self.nodes[node].id_right;

                let max_left = self.update(start, end, t, left_id).unwrap();
                let max_right = self.update(start, end, t, right_id).unwrap();

                let max: i32 = std::cmp::max(max_left, max_right);

                if max != self.nodes[node].key {
                    self.nodes[node].key = max;
                    /*println!("ASSIGNED NODE : {:?} to node {} range {}-{} comparing -> {}-{}({}) & {}-{}({})",
                        max, 
                        node,
                        self.nodes[node].range.start,
                        self.nodes[node].range.end,
                        self.nodes[left_id.unwrap()].range.start,
                        self.nodes[left_id.unwrap()].range.end,
                        max_left.unwrap(),
                        self.nodes[right_id.unwrap()].range.start,
                        self.nodes[right_id.unwrap()].range.end,
                        max_right.unwrap());*/
                }

                return Some(self.nodes[node].key);
            }
        }

        None
    }

    pub fn print_tree(&self) {
        if self.nodes.is_empty() {
            println!("Tree is empty.");
        } else {
            println!("!!!SEGTREE!!!");
            self.print_node(0, 0);
            //println!("!!!LAZYTREE!!!");
            //self.print_lazy_node(0, 0);
        }
    }

    // Recursive helper function to print each node and its children
    fn print_node(&self, node_index: usize, depth: usize) {
        if let Some(node) = self.nodes.get(node_index) {
            // Print the current node with the label and indentation
            println!(
                "{} {} - {}, range = {}-{} pos: {} left: {:?} && right: {:?}",
                "    ".repeat(depth*2),
                depth,
                node.key,
                node.range.start,
                node.range.end,
                node_index,
                node.id_left,
                node.id_right
            );

            // Print the left child with label "Child1" or "GrandchildX" based on depth
            if let Some(left_index) = node.id_left {
                self.print_node(left_index , depth + 1);
            }

            // Print the right child with label "Child2" or "GrandchildX" based on depth
            if let Some(right_index) = node.id_right {
                self.print_node(right_index, depth + 1);
            }
        }
    }

    fn print_lazy_node(&self, node_index: usize, depth: usize) {
        if let Some(node) = self.lazy_nodes[node_index] {
            // Print the current node with the label and indentation
            println!(
                "{}- {:?}, range = {}-{} pos: {} left:{:?} & right:{:?}",
                "    ".repeat(depth*2),
                node,
                self.nodes[node_index].range.start,
                self.nodes[node_index].range.end,
                node_index,
                self.nodes[node_index].id_left,
                self.nodes[node_index].id_right
            );

            // Print the left child with label "Child1" or "GrandchildX" based on depth
            if let Some(left_index) = self.nodes[node_index].id_left {
                self.print_lazy_node(left_index , depth + 1);
            }

            // Print the right child with label "Child2" or "GrandchildX" based on depth
            if let Some(right_index) = self.nodes[node_index].id_right {
                self.print_lazy_node(right_index, depth + 1);
            }
        }
    }
    pub fn validate_tree(&self) -> Result<(), String> {
        for (index, node) in self.nodes.iter().enumerate() {
            // Validate left child
            if let Some(left_id) = node.id_left {
                if left_id >= self.nodes.len() {
                    return Err(format!(
                        "Node {}: Left child index {} is out of bounds.",
                        index, left_id
                    ));
                }
                let left_child = &self.nodes[left_id];
                if left_child.range.start < node.range.start || left_child.range.end > node.range.end {
                    return Err(format!(
                        "Node {}: Left child range {:?}-{:?} is inconsistent with parent range {:?}-{:?}.",
                        index, left_child.range.start, left_child.range.end, node.range.start, node.range.end
                    ));
                }
            }
    
            // Validate right child
            if let Some(right_id) = node.id_right {
                if right_id >= self.nodes.len() {
                    return Err(format!(
                        "Node {}: Right child index {} is out of bounds.",
                        index, right_id
                    ));
                }
                let right_child = &self.nodes[right_id];
                if right_child.range.start < node.range.start || right_child.range.end > node.range.end {
                    return Err(format!(
                        "Node {}: Right child range {:?}-{:?} is inconsistent with parent range {:?}-{:?}.",
                        index, right_child.range.start, right_child.range.end, node.range.start, node.range.end
                    ));
                }
            }
    
            // Validate key (optional)
            let left_key = node.id_left;
            let right_key = node.id_right;
            if left_key.is_some() { 
                if node.key != max(self.nodes[left_key.unwrap()].key, self.nodes[right_key.unwrap()].key) {
                    return Err(format!(
                        "Node {}: Key {} does not match the max coming from keys ({} , {}).",
                        index, node.key, left_key.unwrap(), right_key.unwrap()
                    ));
                }
            }
        }
        Ok(())
    }
}



pub struct IsThere{
    nodes: Vec<RangeNode>,
    lazy_nodes: Vec<Option<i32>>
}

fn tree_init_zero(start: usize, end: usize, level: usize) -> Vec<LevelNode> {
    if start == end {
        return 
            vec![LevelNode::new(
                RangeNode::new(0, Range::new(start, end)),
                level,
            )];
    }

    let mid = (start + end) / 2;
    let left_seg_tree = tree_init_zero(start, mid, level + 1);
    let right_seg_tree = tree_init_zero(mid + 1, end, level + 1);

    let mut ret_arr: Vec<LevelNode> = Vec::with_capacity((2 * left_seg_tree.len()) + 1);
    ret_arr.push(LevelNode::new(
        RangeNode::new(0, Range::new(start, end)),
        level,
    ));
    merge(&left_seg_tree, &right_seg_tree, &mut ret_arr);

    ret_arr
}


impl IsThere {

    fn new(intervals: i32) -> Self {

        let ranges = tree_init_zero(0, (intervals as usize)-1, 0);
        let mut lazy_tree: Vec<Option<i32>> = Vec::with_capacity(ranges.len());
        let mut seg_tree: Vec<RangeNode> = Vec::with_capacity(ranges.len());

        let mut nonode = 0;

        for (i, node) in ranges.iter().enumerate() {
            let mut new_node = node.range_node.clone_node();

            if new_node.range.end - new_node.range.start == 0 {
                new_node.id_left = None;
                new_node.id_right = None;
                nonode += 1;
            } else {
                new_node.id_left =  Some(((i - nonode) * 2) + 1);
                new_node.id_right = Some(((i - nonode) * 2) + 2);
            }

            seg_tree.push(new_node);
            lazy_tree.push(None);
        }

        Self {
            nodes: seg_tree,
            lazy_nodes: lazy_tree,
        }

    }
}