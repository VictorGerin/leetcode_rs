use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
mod solution;
use solution::Solution;

fn main() -> Result<(), String> {
    let chars = read_input("longest_word_in_dictionary_524/src/input.txt")?;

    let values: Vec<Value> = ValueIterator::new(chars)
        .collect::<Result<_, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let s = caso_teste[0].as_string().ok_or(format!("Invalid input for s: {:?}", caso_teste[0]))?.to_string();
            let dictionary = caso_teste[1].as_vec(|v| {
                v.as_string().ok_or(format!("Invalid dictionary entry: {:?}", v)).map(|s| s.to_string())
            }).ok_or(format!("Expected vector for dictionary, got: {:?}", caso_teste[1]))?;
            let expected = caso_teste[2].as_string().ok_or(format!("Invalid expected result: {:?}", caso_teste[2]))?.to_string();

            let result = Solution::find_longest_word(s, dictionary);
            assert_eq!(result, expected, "Expected result to be '{}', but got '{}'", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");

    Ok(())
}
