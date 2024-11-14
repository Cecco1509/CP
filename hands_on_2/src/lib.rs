use std::cmp::max;

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
    key: Option<i32>,
    range: Range,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl RangeNode  {
    fn new(key: Option<i32>, range: Range) -> Self {
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
    lazy_nodes: Vec<RangeNode>,
}

fn tree_init(arr: &Vec<i32>, start: usize, end: usize, level: usize) -> (Vec<LevelNode>, i32) {
    if start == end {
        return (
            vec![LevelNode::new(
                RangeNode::new(Some(arr[start]), Range::new(start, end)),
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
        RangeNode::new(Some(maximum), Range::new(start, end)),
        level,
    ));
    merge(&left_seg_tree, &right_seg_tree, &mut ret_arr);

    // for node in ret_arr.iter() {
    //     print!(
    //         "({} range: {}-{})",
    //         node.level, node.range_node.range.start, node.range_node.range.end
    //     );
    // }

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
    pub fn new(arr: Vec<i32>) -> Self {
        let (ranges, _) = tree_init(&arr, 0, arr.len() - 1, 0);
        let mut lazy_tree: Vec<RangeNode> = Vec::with_capacity(ranges.len());
        let mut seg_tree: Vec<RangeNode> = Vec::with_capacity(ranges.len());

        for (i, node) in ranges.iter().enumerate() {
            let mut new_node = node.range_node.clone_node();

            if (i * 2 + 2) > ranges.len() {
                new_node.id_left = None;
                new_node.id_right = None
            } else {
                new_node.id_left = Some((i * 2) + 1);
                new_node.id_right = Some((i * 2) + 2);
            }

            let mut lazy_node = RangeNode::new(
                None,
                Range::new(node.range_node.range.start, node.range_node.range.end),
            );
            lazy_node.id_left = new_node.id_left;
            lazy_node.id_right = new_node.id_right;

            seg_tree.push(new_node);
            lazy_tree.push(lazy_node);
        }

        Self {
            nodes: seg_tree,
            lazy_nodes: lazy_tree,
        }
    }

    pub fn query(&mut self, query: usize, start: usize, end: usize, t: i32) -> Option<i32> {
        if query == 0 {
            return self.update(start - 1, end - 1, t, Some(0));
        }
        self.max(start - 1, end - 1, Some(0))
    }

    fn max(&mut self, start: usize, end: usize, node: Option<usize>) -> Option<i32> {
        if let Some(node) = node {
            if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
                return None;
            }

            if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {

                if self.lazy_nodes[node].key.is_some() {
                    self.nodes[node].key = self.lazy_nodes[node].key;

                    if let Some(id_left) = self.nodes[node].id_left {
                        self.lazy_nodes[id_left].key = self.lazy_nodes[node].key;
                    }

                    if let Some(id_right) = self.nodes[node].id_right {
                        self.lazy_nodes[id_right].key = self.lazy_nodes[node].key;
                    }

                    self.lazy_nodes[node].key = None;
                }

                return self.nodes[node].key;
            }

            if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {
                let max_left = self.max(start, end, self.nodes[node].id_left);
                let max_right = self.max(start, end, self.nodes[node].id_right);

                if max_left.is_none() && max_right.is_none() {
                    return self.nodes[node].key;
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

    fn update(&mut self, start: usize, end: usize, t: i32, node: Option<usize>) -> Option<i32> {

        if let Some(node) = node {

            // Nessuna sovrapposizione
            if self.nodes[node].range.start > end || self.nodes[node].range.end < start {
                return None;
            }

            // Sovrapposizione completa
            if self.nodes[node].range.start >= start && self.nodes[node].range.end <= end {
                
                if self.lazy_nodes[node].key.is_some() {

                    if self.lazy_nodes[node].key.unwrap() < t { 
                        return self.nodes[node].key;
                    }
                        
                    self.lazy_nodes[node].key = Some(t);
                    

                }else {

                    if self.nodes[node].key.unwrap() < t { 
                        return self.nodes[node].key;
                    }

                    self.lazy_nodes[node].key = Some(t);

                }
                
                return self.lazy_nodes[node].key;
            }

            // Sovrapposizione parziale
            if self.nodes[node].range.start <= start || self.nodes[node].range.end >= end {
                let max_left = self.update(start, end, t, self.nodes[node].id_left);
                let max_right = self.update(start, end, t, self.nodes[node].id_right);

                if max_left.is_none() && max_right.is_none() {
                    print!("HEY");
                    return self.nodes[node].key;
                }

                let max: Option<i32> ;

                if max_left.is_none() {
                    max = max_right;
                } else if max_right.is_none() {
                    max = max_left;
                }else{
                    max = std::cmp::max(max_left, max_right);
                }

                if max.unwrap() != self.nodes[node].key.unwrap() {
                    self.lazy_nodes[node].key = max;
                    return self.lazy_nodes[node].key;
                }

                return self.nodes[node].key;
            }
        }

        None
    }

    pub fn print_tree(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!(
                "Node: {}, Range: ({}, {}), max: {}, left: {:?}, right: {:?}",
                i, node.range.start, node.range.end, node.key.unwrap(), node.id_left, node.id_right
            );
        }
    }
}
