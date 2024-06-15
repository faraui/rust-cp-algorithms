fn find_prefix(&self, v: usize, l: usize, r: usize, x: usize) -> isize {
    if l == r {
        if self.t[v] == x {
            l as isize
        } else {
            -1
        }
    } else {
        let m = (l + r) / 2;
        if self.t[2 * v] <= x {
            self.find_prefix(2 * v, l, m, x)
        } else {
            self.find_prefix(2 * v + 1, m + 1, r, x)
        }
    }
}

fn find_segment(&self, v: usize, l: usize, r: usize, ql: usize, x: usize) -> isize {
    if r < ql {
        -1
    } else if l >= ql {
        if self.t[v] <= x {
            self.find_prefix(v, l, r, x)
        } else {
            -2
        }
    } else {
        let m = (l + r) / 2;
        let left = self.find_segment(2 * v, l, m, ql, x);
        if left != -2 {
            left
        } else {
            self.find_segment(2 * v + 1, m + 1, r, ql, x)
        }
    }
}

fn find_prefix_max(&self, v: usize, l: usize, r: usize, x: usize) -> isize {
    if l == r {
        if self.t[v] > x {
            l as isize
        } else {
            -1
        }
    } else {
        let m = (l + r) / 2;
        if self.t[2 * v] > x {
            self.find_prefix_max(2 * v, l, m, x)
        } else {
            self.find_prefix_max(2 * v + 1, m + 1, r, x)
        }
    }
}

fn find_segment_max(&self, v: usize, l: usize, r: usize, ql: usize, x: usize) -> isize {
    if r < ql {
        -1
    } else if l >= ql {
        if self.t[v] > x {
            self.find_prefix_max(v, l, r, x)
        } else {
            -1
        }
    } else {
        let m = (l + r) / 2;
        let left = self.find_segment_max(2 * v, l, m, ql, x);
        if left != -1 {
            left
        } else {
            self.find_segment_max(2 * v + 1, m + 1, r, ql, x)
        }
    }
}
