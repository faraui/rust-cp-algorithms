struct Line {
    k: f64,
    b: f64,
}

impl Line {
    fn new(k: f64, b: f64) -> Self {
        Line { k, b }
    }

    fn get(&self, x: f64) -> f64 {
        self.k * x + self.b
    }
}

fn cross(a: &Line, b: &Line) -> f64 {
    (b.b - a.b) / (a.k - b.k)
}

fn add(lines: &mut Vec<Line>, dots: &mut Vec<f64>, l: Line) {
    while !lines.is_empty() && lines.last().unwrap().get(dots.last().unwrap()) >= l.get(dots.last().unwrap()) {
        lines.pop();
        dots.pop();
    }
    if lines.is_empty() {
        dots.push(-1e9);
    } else {
        dots.push(cross(lines.last().unwrap(), &l));
    }
    lines.push(l);
}

fn find(dots: &Vec<f64>, x: f64) -> f64 {
    let i = dots.upper_bound(&x).unwrap() - 1;
    let l = &dots[i];
    l.get(x)
}
