
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
    let (left_nodes, left_max) = tree_init(arr, start, mid, level + 1);
    let (right_nodes, right_max) = tree_init(arr, mid + 1, end, level + 1);
    let maximum: i32 = i32::max(left_max, right_max);

    let mut ret_arr: Vec<LevelNode> = Vec::with_capacity((left_nodes.len() + right_nodes.len()) + 1);
    ret_arr.push(LevelNode::new(
        RangeNode::new(maximum, Range::new(start, end)),
        level,
    ));
    merge(&left_nodes, &right_nodes, &mut ret_arr);

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

    merged_arr.extend(left[i..].iter().map(LevelNode::clone_node));
    merged_arr.extend(right[j..].iter().map(LevelNode::clone_node));
}

impl MinMax {
    /* Complexity = O(n log n) used merge sort to build the segment tree*/
    pub fn new(arr: Vec<i32>) -> Self {
        let (ranges, _) = tree_init(&arr, 0, arr.len() - 1, 0);
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
                new_node.id_left = Some(((i - nonode) * 2) + 1);
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

                self.propagate(node, new_val);

                self.lazy_nodes[node] = None;
            }
        }
    }

    fn propagate(&mut self, node: usize, t: i32) {
        if self.nodes[node].id_left.is_none() {
            return;
        }

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
                }

                return Some(self.nodes[node].key);
            }
        }

        None
    }
}

pub struct IsThere {
    nodes: Vec<URangeNode>,
    lazy_nodes: Vec<u128>,
}

fn tree_init_zero(start: usize, end: usize, level: usize) -> Vec<LevelNode> {
    if start == end {
        return vec![LevelNode::new(
            RangeNode::new(1, Range::new(start, end)),
            level,
        )];
    }

    let mid = (start + end) / 2;
    let left_seg_tree = tree_init_zero(start, mid, level + 1);
    let right_seg_tree = tree_init_zero(mid + 1, end, level + 1);

    let mut ret_arr: Vec<LevelNode> = Vec::with_capacity((2 * left_seg_tree.len()) + 1);
    ret_arr.push(LevelNode::new(
        RangeNode::new(1, Range::new(start, end)),
        level,
    ));
    merge(&left_seg_tree, &right_seg_tree, &mut ret_arr);

    ret_arr
}

pub struct URangeNode {
    key: u128,
    range: Range,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl URangeNode {
    fn new(key: u128, range: Range) -> Self {
        Self {
            key,
            range,
            id_left: None,
            id_right: None,
        }
    }

    pub fn clone_node(&self) -> URangeNode {
        URangeNode::new(self.key, Range::new(self.range.start, self.range.end))
    }
}

impl IsThere {
    pub fn new(interval: u128) -> Self {
        let ranges = tree_init_zero(0, (interval as usize) - 1, 0);
        let mut lazy_tree: Vec<u128> = Vec::with_capacity(ranges.len());
        let mut seg_tree: Vec<URangeNode> = Vec::with_capacity(ranges.len());

        let mut nonode = 0;

        for (i, node) in ranges.iter().enumerate() {
            let mut new_node = URangeNode::new(
                1,
                Range::new(node.range_node.range.start, node.range_node.range.end),
            );

            if new_node.range.end - new_node.range.start == 0 {
                new_node.id_left = None;
                new_node.id_right = None;
                nonode += 1;
            } else {
                new_node.id_left = Some(((i - nonode) * 2) + 1);
                new_node.id_right = Some(((i - nonode) * 2) + 2);
            }

            seg_tree.push(new_node);
            lazy_tree.push(0);
        }

        Self {
            nodes: seg_tree,
            lazy_nodes: lazy_tree,
        }
    }

    fn update_node(&mut self, node: usize) {
        if self.lazy_nodes[node] > 0 {
            self.nodes[node].key <<= self.lazy_nodes[node];

            self.propagate(node, self.lazy_nodes[node]);
            self.lazy_nodes[node] = 0;
        }
    }

    fn propagate(&mut self, node: usize, t: u128) {
        if self.nodes[node].id_left.is_none() {
            return;
        }

        let left_id = self.nodes[node].id_left.unwrap();
        let right_id = self.nodes[node].id_right.unwrap();

        self.lazy_nodes[left_id] += t;
        self.lazy_nodes[right_id] += t;
    }

    pub fn query(&mut self, query: usize, i: usize, j: usize, k: u128) -> i8 {
        let p = 2u128.pow(k as u32);
        if query == 1 {
            let ret = self.is_there(i, j, p, 0) as i8;

            if ret > 0 {
                return 1;
            }
            return 0;
        }

        self.update(i, j, 0) as i8
    }

    fn is_there(&mut self, start: usize, end: usize, k: u128, node: usize) -> u128 {
        self.update_node(node);

        if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
            return 0;
        }

        if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {
            return self.nodes[node].key & k;
        }

        if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {
            let left_overlap = self.is_there(start, end, k, self.nodes[node].id_left.unwrap());
            let right_overlap = self.is_there(start, end, k, self.nodes[node].id_right.unwrap());

            return right_overlap | left_overlap;
        }

        0
    }

    fn update(&mut self, start: usize, end: usize, node: usize) -> u128 {
        self.update_node(node);

        if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
            return self.nodes[node].key;
        }

        if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {
            self.nodes[node].key <<= 1;
            self.propagate(node, 1);
            return self.nodes[node].key;
        }

        if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {
            let left_overlap = self.update(start, end, self.nodes[node].id_left.unwrap());
            let right_overlap = self.update(start, end, self.nodes[node].id_right.unwrap());

            self.nodes[node].key = right_overlap | left_overlap;
            return self.nodes[node].key;
        }

        0
    }
}
