pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points.windows(2).map(|pair| {
            let [x1, y1] = [pair[0][0], pair[0][1]];
            let [x2, y2] = [pair[1][0], pair[1][1]];
            (x2 - x1).abs().max((y2 - y1).abs())
        }).sum()
    }
}
