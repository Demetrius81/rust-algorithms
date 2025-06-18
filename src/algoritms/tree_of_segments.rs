#![allow(dead_code)]

struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let tree = vec![0; 4 * n];
        let mut seg_tree = SegmentTree { n, tree };
        seg_tree.build(arr, 0, 0, n - 1);
        seg_tree
    }

    fn build(&mut self, arr: &[i32], node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = arr[left];
        } else {
            let mid = (left + right) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;

            self.build(arr, left_child, left, mid);
            self.build(arr, right_child, mid + 1, right);

            self.tree[node] = self.tree[left_child] + self.tree[right_child];
        }
    }

    fn query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if ql > right || qr < left {
            return 0;
        }
        if ql <= left && qr >= right {
            return self.tree[node];
        }
        let mid = (left + right) / 2;
        let left_sum = self.query(2 * node + 1, left, mid, ql, qr);
        let right_sum = self.query(2 * node + 2, mid + 1, right, ql, qr);
        left_sum + right_sum
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, new_val: i32) {
        if left == right {
            self.tree[node] = new_val;
        } else {
            let mid = (left + right) / 2;
            if idx <= mid {
                self.update(2 * node + 1, left, mid, idx, new_val);
            } else {
                self.update(2 * node + 2, mid + 1, right, idx, new_val);
            }
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    pub fn sum(&self, ql: usize, qr: usize) -> i32 {
        self.query(0, 0, self.n - 1, ql, qr)
    }

    pub fn modify(&mut self, idx: usize, new_val: i32) {
        self.update(0, 0, self.n - 1, idx, new_val);
    }
}

pub fn run() {
    let arr = [1, 3, 5, 7, 9, 11];
    let mut seg_tree = SegmentTree::new(&arr);

    let mut sum = seg_tree.sum(1, 3);
    assert!(sum == 15);
    println!("Summ from 1 to 3: {}", sum);

    seg_tree.modify(1, 10);
    sum = seg_tree.sum(1, 3);
    assert!(sum == 22);
    println!("Amount from 1 to 3 after change: {}", sum);
}
