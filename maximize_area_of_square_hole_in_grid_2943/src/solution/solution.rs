use std::sync::Arc;

pub struct Solution;

impl Solution {

    fn find_max_sequence(bars: Arc<[i32]>) -> Option<i32> {
        let mut iter = bars.iter();
        let mut current_number = iter.next().copied()?;
        let mut current_sequence = 1;
        let mut max_sequence = 0;
        for x in iter {
            if *x != (current_number + 1) {
                max_sequence = max_sequence.max(current_sequence);
                current_sequence = 1;
            } else {
                current_sequence += 1;
            }
            current_number = *x;
        }
        Some(max_sequence.max(current_sequence))
    }

    pub fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars.clone();
        h_bars.sort();
        let mut v_bars = v_bars.clone();
        v_bars.sort();

        let max_h_bar_sequence = Self::find_max_sequence(h_bars.into()).unwrap_or(0) + 1;
        let max_v_bar_sequence = Self::find_max_sequence(v_bars.into()).unwrap_or(0) + 1;

        let side = max_h_bar_sequence.min(max_v_bar_sequence);
        let qudrado: i32 = side * side;
        
        qudrado
    }
}
