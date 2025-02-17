use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::{to_value as to_json_result};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const TOO_FEW_ERROR_MESSAGE: &str = "Cannot solve for fewer than 2 matchsticks";

#[derive(Serialize, Debug, PartialEq)]
struct CalcResult {
    result: String,
    length: usize
}

#[derive(Serialize, Debug, PartialEq)]
struct OkResult {
    correct: bool
}

#[derive(Serialize, Debug, PartialEq)]
struct ErrorResult<'a> {
    error: &'a str
}

#[wasm_bindgen]
pub fn check_largest(number_of_matchsticks: usize, input: String) -> Result<JsValue, JsValue> {
    translate_check_result(calculate_largest_internal(number_of_matchsticks), &input)
}

#[wasm_bindgen]
pub fn calculate_largest(number_of_matchsticks: usize) -> Result<JsValue, JsValue> {
    translate_calculation_result(calculate_largest_internal(number_of_matchsticks))
}

fn calculate_largest_internal(number_of_matchsticks: usize) -> Result<CalcResult, ErrorResult<'static>> {
    if number_of_matchsticks < 2 {
        Err(ErrorResult { error: TOO_FEW_ERROR_MESSAGE })
    } else {
        let (length, remainder) = get_digits_and_remainder(&number_of_matchsticks, 2);
        let mut result = "1".repeat(length - 1);
        if remainder == 0 {
            result.insert_str(0, "1");
        } else {
            result.insert_str(0, "7");
        }
        Ok(CalcResult { result, length })
    }
}

#[wasm_bindgen]
pub fn check_smallest(number_of_matchsticks: usize, input: String) -> Result<JsValue, JsValue> {
    translate_check_result(calculate_smallest_internal(number_of_matchsticks), &input)
}

#[wasm_bindgen]
pub fn calculate_smallest(number_of_matchsticks: usize) -> Result<JsValue, JsValue> {
    translate_calculation_result(calculate_smallest_internal(number_of_matchsticks))
}

fn calculate_smallest_internal(number_of_matchsticks: usize) -> Result<CalcResult, ErrorResult<'static>> {
    if number_of_matchsticks < 2 {
        Err(ErrorResult { error: TOO_FEW_ERROR_MESSAGE })
    } else {
        // adding a leading zero is at 2, 8, 14, 20... i.e. offset by 2 from the 6x tables
        let zero_digits = (number_of_matchsticks - 2) / 6;
        let remainder = number_of_matchsticks % 6;
        let mut result = "0".repeat(zero_digits);

        match remainder {
            2 => result.push_str("1"),
            3 => result.push_str("7"),
            4 => result.push_str("4"),
            5 => result.push_str("2"),
            0 => result.push_str("0"), // 6 sticks
            1 => result.push_str("8"), // 7 sticks
            r => panic!("Should not get {} as a remainder", r)
        };
        Ok(CalcResult { result, length: zero_digits + 1 })
    }
}

#[wasm_bindgen]
pub fn check_smallest_no_leading_zeroes(number_of_matchsticks: usize, input: String) -> Result<JsValue, JsValue> {
    translate_check_result(calculate_smallest_no_leading_zeroes_internal(number_of_matchsticks), &input)
}

#[wasm_bindgen]
pub fn calculate_smallest_no_leading_zeroes(number_of_matchsticks: usize) -> Result<JsValue, JsValue> {
    translate_calculation_result(calculate_smallest_no_leading_zeroes_internal(number_of_matchsticks))
}

//noinspection RsNonExhaustiveMatch
fn calculate_smallest_no_leading_zeroes_internal(number_of_matchsticks: usize) -> Result<CalcResult, ErrorResult<'static>> {
    if number_of_matchsticks < 2 {
        Err(ErrorResult { error: "Cannot solve for fewer than 2 matchsticks" })
    } else {
        let (digits, remainder) = get_digits_and_remainder(&number_of_matchsticks, 7);
        let (result, final_digits) = match remainder {
            0 => {
                ("8".repeat(digits), digits)
            },
            1 => (
                format!("10{}", "8".repeat(digits.checked_sub(1).unwrap_or(0))),
                digits + 1
            ),
            2 => (
                format!("1{}", "8".repeat(digits)),
                digits + 1
            ),
            3 if digits >= 2 => (
                format!("200{}", "8".repeat(digits - 2)),
                digits + 1
            ),
            3 if digits == 1 => (
                "22".to_string(),
                2
            ),
            3 if digits == 0 => (
                "7".to_string(),
                1
            ),
            4 if digits >= 1 => (
                format!("20{}", "8".repeat(digits.checked_sub(1).unwrap_or(0))),
                digits + 1
            ),
            4 if digits == 0 => (
                "4".to_string(),
                1
            ),
            5 => (
                format!("2{}", "8".repeat(digits)),
                digits + 1
            ),
            6 if digits == 0 => (
                "14".to_string(),
                2
            ),
            6 if digits >= 1 => (
                format!("80{}", "8".repeat(digits.checked_sub(1).unwrap_or(0))),
                digits + 1
            ),
            r => panic!("Should not get {} as a remainder", r)
        };
        Ok(CalcResult { result, length: final_digits })
    }
}

fn get_digits_and_remainder(number_of_matchsticks: &usize, division: usize) -> (usize, usize) {
    (number_of_matchsticks / division, number_of_matchsticks % division)
}

fn translate_calculation_result(result: Result<CalcResult, ErrorResult>) -> Result<JsValue, JsValue> {
    result.map(|x| to_json_result(&x).unwrap()).map_err(|x| to_json_result(&x).unwrap())
}

fn translate_check_result(result: Result<CalcResult, ErrorResult>, input: &String) -> Result<JsValue, JsValue> {
    result.map(|x| {
        to_json_result(
            &OkResult {
                correct: x.result.eq(input.trim())
            }
        ).unwrap()
    }).map_err(|x| to_json_result(&x).unwrap())
}

// Run these tests using one of the following:
//
//  macOS: "cargo test --target=aarch64-apple-darwin"
//  Windows: "cargo test --target=aarch64-pc-windows-msvc"
#[cfg(all(test, not(target_family = "wasm")))]
// #[cfg(test)]
mod tests {
    use proptest::proptest;
    use rstest::rstest;
    use crate::{calculate_largest_internal, calculate_smallest_no_leading_zeroes_internal, calculate_smallest_internal, ErrorResult, TOO_FEW_ERROR_MESSAGE};

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_get_largest_fails_for_less_than_two_sticks(#[case] sticks: usize) {
        assert_eq!(calculate_largest_internal(sticks), Err(ErrorResult { error: TOO_FEW_ERROR_MESSAGE }))
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_get_smallest_fails_for_less_than_two_sticks(#[case] sticks: usize) {
        assert_eq!(calculate_smallest_internal(sticks), Err(ErrorResult { error: TOO_FEW_ERROR_MESSAGE }))
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_get_smallest_no_leading_zeroes_fails_for_less_than_two_sticks(#[case] sticks: usize) {
        assert_eq!(calculate_smallest_no_leading_zeroes_internal(sticks), Err(ErrorResult { error: TOO_FEW_ERROR_MESSAGE }))
    }

    #[rstest]
    #[case(2, 1, "1")]
    #[case(3, 1, "7")]
    #[case(4, 1, "4")]
    #[case(5, 1, "2")]
    #[case(6, 1, "0")]
    #[case(7, 1, "8")]
    #[case(8, 2, "01")]
    #[case(9, 2, "07")]
    #[case(10, 2, "04")]
    #[case(11, 2, "02")]
    #[case(12, 2, "00")]
    #[case(13, 2, "08")]
    #[case(14, 3, "001")]
    #[case(15, 3, "007")]
    #[case(16, 3, "004")]
    #[case(17, 3, "002")]
    #[case(18, 3, "000")]
    #[case(19, 3, "008")]
    #[case(20, 4, "0001")]
    #[case(25, 4, "0008")]
    #[case(96, 16, "0000000000000000")]
    #[case(97, 16, "0000000000000008")]
    #[case(98, 17, "00000000000000001")]
    #[case(99, 17, "00000000000000007")]
    #[case(100, 17, "00000000000000004")]
    #[case(101, 17, "00000000000000002")]
    fn test_get_smallest_specific_examples(#[case] sticks: usize, #[case] digits: usize, #[case] value: &str) {
        let result = calculate_smallest_internal(sticks).unwrap();
        assert_eq!(result.result, value, "For {} matchsticks, result is {}, expected {}", sticks, result.result, value);
        assert_eq!(result.length, digits, "For {} matchsticks, {} digits were returned, but we expected {}", sticks, result.length, digits);
    }


    #[rstest]
    #[case(2,  1, "1")] // 2 matchsticks create a 1
    #[case(3,  1, "7")] // 3 matchsticks create a 7
    #[case(4,  1, "4")] // 4 matchsticks create a 4
    #[case(5,  1, "2")] // 5 matchsticks create a 2 (or 3, 5, 6, 9)
    #[case(6,  2, "14")] // We can't have a leading zero, so 14 is the lowest two digit combination we can have!
    #[case(7,  1, "8")] // 8 is the only one digit combination
    #[case(8,  2, "10")] // 2 + 6 -- leading 1
    #[case(9,  2, "18")] // 2 + 7 gives the leading 1
    #[case(10,  2, "22")] // 10 -> 5 + 5. Can't get a 1 (would leave 8 left over), 4 + 6 would give 40, 3 + 7 => 78
    #[case(11,  2, "20")] // 11 -> 5 + 6. Can't get a 1 (would leave 9 left over)
    #[case(12,  2, "28")] // Last special case, 12 -> 5 + 7, 6 + 6 would be 00, only leading zeroes!
    #[case(13,  2, "80")] // 7 + 6.
    #[case(14,  2, "88")] // 7 + 7
    #[case(15,  3, "108")] // 2 + 6 + 7
    #[case(16,  3, "188")] // 2 + 7 + 7
    #[case(17,  3, "200")] // 5 + 6 + 6 gives a smaller result than 5 + 5 + 7 (228). No way to get a 1 at the front (can't make up 15 with two digits)
    #[case(18,  3, "208")] // 5 + 6 + 7 -- we can't get a 1, so a leading 2 is our best bet. A zero being next is the best option, leaving an 8.
    #[case(19,  3, "288")] // 5 + 7 + 7 -- better than 7 + 6 + 6 which would be 800.
    #[case(20,  3, "808")] // 7 + 6 + 7
    #[case(21,  3, "888")] // 7 + 7 + 7
    #[case(22,  4, "1088")] // 2 + 6 + 7 + 7 = 22
    #[case(23,  4, "1888")] // 2 + 7 + 7 + 7 = 23
    #[case(24,  4, "2008")] // 5 + 6 + 6 + 7 = 24
    #[case(25,  4, "2088")] // 5 + 6 + 7 + 7 = 25
    #[case(26,  4, "2888")] // 5 + 7 + 7 + 7 = 26
    #[case(27,  4, "8088")] // 7 + 6 + 7 + 7 = 27
    #[case(28,  4, "8888")] // 7 + 7 + 7 + 7 = 28 (and then the cycle repeats)
    #[case(98,  14, "88888888888888")]
    #[case(99,  15, "108888888888888")]
    #[case(100, 15, "188888888888888")]
    #[case(101, 15, "200888888888888")]
    #[case(102, 15, "208888888888888")]
    #[case(103, 15, "288888888888888")]
    #[case(104, 15, "808888888888888")]
    fn test_get_smallest_no_leading_zeroes_specific_examples(#[case] sticks: usize, #[case] digits: usize, #[case] value: &str) {
        let result = calculate_smallest_no_leading_zeroes_internal(sticks).unwrap();
        assert_eq!(result.result, value, "For {} matchsticks, result is {}, expected {}", sticks, result.result, value);
        assert_eq!(result.length, digits, "For {} matchsticks, {} digits were returned, but we expected {}", sticks, result.length, digits);
    }

    proptest! {

        #[test]
        fn test_get_largest_even(digits in 1usize..100000) {
            let matchsticks = digits * 2;
            let expected_string = "1".repeat(digits);
            let result = calculate_largest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits, "Even number of digits, should get {} digits for {} matchsticks, but got {} digits", digits, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_largest_odd(digits in 1usize..100000) {
            let matchsticks = (digits * 2) + 1;
            let expected_string = format!("7{}", "1".repeat(digits - 1));
            let result = calculate_largest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits, "Odd number of digits, should get {} digits for {} matchsticks, but got {} digits ", digits, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_0(digits in 1usize..100000) {
            let matchsticks = digits * 6;
            let expected_string = "0".repeat(digits);
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits, "Multiple of six digits, should get {} digits for {} matchsticks, but got {} digits ", digits, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_1(digits in 1usize..100000) {
            let matchsticks = digits * 6 + 1;
            let expected_string = format!("{}8", "0".repeat(digits - 1));
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits, "Remainder 1, should get {} digits for {} matchsticks, but got {} digits ", digits, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_2(digits in 1usize..100000) {
            let matchsticks = digits * 6 + 2;
            let expected_string = format!("{}1", "0".repeat(digits));
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits + 1, "Remainder 2, should get {} digits for {} matchsticks, but got {} digits ", digits + 1, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_3(digits in 1usize..100000) {
            let matchsticks = digits * 6 + 3;
            let expected_string = format!("{}7", "0".repeat(digits));
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits + 1, "Remainder 3, should get {} digits for {} matchsticks, but got {} digits ", digits + 1, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_4(digits in 1usize..100000) {
            let matchsticks = digits * 6 + 4;
            let expected_string = format!("{}4", "0".repeat(digits));
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits + 1, "Remainder 4, should get {} digits for {} matchsticks, but got {} digits ", digits + 1, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

        #[test]
        fn test_get_smallest_5(digits in 1usize..100000) {
            let matchsticks = digits * 6 + 5;
            let expected_string = format!("{}2", "0".repeat(digits));
            let result = calculate_smallest_internal(matchsticks).unwrap();
            assert_eq!(result.length, digits + 1, "Remainder 5, should get {} digits for {} matchsticks, but got {} digits ", digits + 1, matchsticks, result.length);
            assert_eq!(result.result, expected_string);
        }

    }
}