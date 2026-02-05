pub struct Solution;

#[derive(Debug, PartialEq)]
enum State {
    Unknown,
    Rising,
    Lowing
}

#[derive(Debug)]
struct Section {
    _init: usize,
    end: usize,
    sum: i64,
    direction: State
}


impl Solution {

    fn split_arr_in_sections(_nums: &Vec<i32>) -> Vec<Section> {
        let mut sections: Vec<Section> = vec![];
        
        use State::{Lowing, Rising, Unknown};

        let mut current_state: State = Unknown;
        let mut first_index: usize = 0;
        let mut current_index: usize = 0;
        let mut current_sum: i64 = 0;
        let mut last_val: i32 = _nums[current_index];

        while current_index < _nums.len() {
            let current_num = _nums[current_index];
            
            match current_state {
                Unknown => {
                    if last_val > current_num {
                        current_state = Lowing;
                    } else if last_val < current_num {
                        current_state = Rising;
                    } else  {
                        current_state = Unknown;
                        current_sum = 0;
                        first_index = current_index;
                    }

                    last_val = _nums[current_index];
                    current_sum += current_num as i64;
                    current_index += 1;
                },
                Rising => {
                    if last_val < current_num {
                        current_state = Rising;

                    } else if last_val > current_num {
                        current_state = Lowing;
                        sections.push(Section {
                            _init: first_index,
                            end: current_index - 1,
                            sum: current_sum,
                            direction: Rising
                        });
                        first_index = current_index - 1;
                        current_sum = last_val as i64;
                    } else {
                        sections.push(Section {
                            _init: first_index,
                            end: current_index - 1,
                            sum: current_sum,
                            direction: Rising
                        });
                        current_state = Unknown;
                        current_sum = 0;
                        first_index = current_index;
                    }

                    last_val = _nums[current_index];
                    current_sum += current_num as i64;
                    current_index += 1;
                },
                Lowing => {
                    if last_val < current_num {
                        current_state = Rising;
                        sections.push(Section {
                            _init: first_index,
                            end: current_index - 1,
                            sum: current_sum,
                            direction: Lowing
                        });
                        first_index = current_index - 1;
                        current_sum = last_val as i64;
                    } else if last_val > current_num {
                        current_state = Lowing;

                    } else {
                        current_state = Unknown;
                        sections.push(Section {
                            _init: first_index,
                            end: current_index - 1,
                            sum: current_sum,
                            direction: Lowing
                        });
                        current_sum = 0;
                        first_index = current_index;
                    }

                    last_val = _nums[current_index];
                    current_sum += current_num as i64;
                    current_index += 1;
                }
            }
        }

        if current_index != first_index {
            sections.push(Section {
                _init: first_index,
                end: current_index - 1,
                sum: current_sum,
                direction: current_state
            });
        }

        sections
    }

    pub fn max_sum_trionic(_nums: Vec<i32>) -> i64 {

        let sections: Vec<Section> = Self::split_arr_in_sections(&_nums);
        //println!("{:?}", sections);
        use State::{Rising, Lowing};


        let mut acc = std::i64::MIN;
        for x in sections.windows(3) {
            if !(x[0].direction == Rising && x[1].direction == Lowing && x[2].direction == Rising) || 
            !(x[0].end == x[1]._init && x[1].end == x[2]._init) {
                continue;
            }

            let first = &x[0];    //Rising
            let seccond = &x[1];  //Lowing
            let tird = &x[2];     //Rising

            let len_first = first.end - first._init + 1;
            let len_third = tird.end - tird._init + 1;
            let mut prefix_first = vec![0i64; len_first];
            for k in 1..len_first {
                prefix_first[k] = prefix_first[k - 1] + _nums[first._init + k - 1] as i64;
            }
            let mut suffix_third = vec![0i64; len_third];
            for k in 1..len_third {
                suffix_third[k] = suffix_third[k - 1] + _nums[tird.end - k + 1] as i64;
            }

            let seccond_sum = seccond.sum - _nums[seccond.end] as i64;
            let base_first = first.sum - _nums[first.end] as i64;

            for i in 0..=(first.end - first._init - 1) {
                let first_sum_i = base_first - prefix_first[i];
                for j in 0..=(tird.end - tird._init - 1) {
                    let trid_sum_j = tird.sum - suffix_third[j];
                    let total = first_sum_i + seccond_sum + trid_sum_j;
                    acc = acc.max(total);
                }
            }
        }
        acc
    }
}
