use std::{collections::HashSet, sync::Arc, vec};

pub struct Solution;

impl Solution {

    fn get_max_side(bars: Arc<[i32]>) -> Option<HashSet<i32>> {
        let mut result: HashSet<i32> = HashSet::new();

        let mut iter = bars.iter().copied();
        let mut current_val = iter.next()?;
        let mut start_at = current_val;

        for side in iter {
            if side != (current_val + 1) {
                result.insert(current_val - start_at + 1);
                start_at = side;
                current_val = side;
            } else {
                current_val = side;
            }
        }

        if current_val - start_at != 0 {
            result.insert(current_val - start_at + 1);
        }

        (!result.is_empty()).then_some(result)
    }

    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut h_bars = h_fences.clone();
        h_bars.push(1);
        h_bars.push(m);
        h_bars.sort_unstable();
        let mut v_bars: Vec<i32> = v_fences.clone();
        v_bars.push(1);
        v_bars.push(n);
        v_bars.sort_unstable();


        println!("{:?} \n {:?}", h_bars, v_bars);

        let h_bars = Self::get_max_side(h_bars.into()).unwrap().iter().max().copied().unwrap_or(0) - 1;
        let v_bars = Self::get_max_side(v_bars.into()).unwrap().iter().max().copied().unwrap_or(0) - 1;

        let min = h_bars.min(v_bars);
        min


        // let teste = vec![1,5,6,7,8,10,11,12,13,14,15,16];
        // let teste = Self::get_max_side(teste.into());


        // todo!()
    }
}
