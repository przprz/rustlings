/// Run with:
/// ```sh
/// cargo run --example q
/// ```

struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {
    pub fn get_curve(&self) -> Option<usize> {
        self.curve
    }

    /// If there is a curve, then increments all
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}

fn main() {
    let mut es= vec![1,2,3];
    let nth = find_nth(&mut es, 2);
    println!("{}", nth);

    let mut tr = TestResult {
        scores: vec![1,2,3  ],
        curve: Some(4),
    };

    tr.apply_curve();
    println!("{:?}", tr.scores);
}

fn find_nth<T: Ord + Clone>(elems: &mut [T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
