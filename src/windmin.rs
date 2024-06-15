use std::collections::VecDeque;

struct SlideMin {
    d: VecDeque<(i32, usize)>,
    l: usize,
    r: usize,
}

impl SlideMin {
    fn new() -> Self {
        SlideMin {
            d: VecDeque::new(),
            l: 0,
            r: 0,
        }
    }

    fn push_back(&mut self, x: i32) {
        while !self.d.is_empty() && self.d.back().unwrap().0 >= x {
            self.d.pop_back();
        }
        self.d.push_back((x, self.r));
        self.r += 1;
    }

    fn pop_front(&mut self) {
        if self.d.front().unwrap().1 == self.l {
            self.d.pop_front();
        }
        self.l += 1;
    }

    fn get_min(&self) -> i32 {
        self.d.front().unwrap().0
    }
}
