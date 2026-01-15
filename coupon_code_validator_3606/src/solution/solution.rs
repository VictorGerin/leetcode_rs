pub struct Solution;

impl Solution {

    fn validade_business(str: &str) -> bool {
        match str {
            "electronics" | "grocery" | "pharmacy" | "restaurant" => true,
            _ => false
        }
    }

    fn valid_code(code: &str) -> bool {
        if code.is_empty() {
            return false;
        }
        code.chars().all(|c| {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
                _ => false
            }
        })
    }

    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut valid_codes= is_active.iter()
        .enumerate()
        .filter_map(|(index, b)| if *b { Some(index) } else { None }) //Only active
        .filter_map(|x| business_line.get(x).map(|str| (x, str)))//Transform in bussines by valid index
        .filter_map(|(x, str)| if Self::validade_business(str) {Some(x)} else {None})//Only valid business active
        .filter_map(|x| code.get(x).map(|str| (x, str))) //Transform in code iter by valid index
        .filter_map(|(_x, str)| if Self::valid_code(str) {Some(str.clone())} else {None})
        .collect::<Vec<String>>();
        valid_codes.sort();
        valid_codes
    }
}
