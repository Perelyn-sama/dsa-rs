fn is_possible(nums: Vec<u32>, target: u32) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let operators: Vec<&str> = vec!["-+", "+-", "--", "++"];
    let mut temp: u32;

    for i in 0..nums.len() {
        match operators[i] {
            "-+" => temp = nums[0] - nums[1] + nums[2],
            "+-" => temp = nums[0] + nums[1] - nums[2],
            "--" => temp = nums[0] - nums[1] - nums[2],
            "++" => temp = nums[0] - nums[1] - nums[2],
            _ => todo!(),
        }
        if temp == target {
            return true;
        }
    }
    false
}

#[test]
fn test_one() {
    let nums: Vec<u32> = vec![9, 3, 5];
    let target: u32 = 11;

    let expected_result = true;
    let actual_result = is_possible(nums, target);

    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_two() {
    let nums: Vec<u32> = vec![11];
    let target: u32 = 11;

    let expected_result = true;
    let actual_result = is_possible(nums, target);

    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_three() {
    let nums: Vec<u32> = vec![10, 10, 10];
    let target: u32 = 20;

    let expected_result = true;
    let actual_result = is_possible(nums, target);

    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_four() {
    let nums: Vec<u32> = vec![10, 12, 18];
    let target: u32 = 20;

    let expected_result = true;
    let actual_result = is_possible(nums, target);

    assert_eq!(expected_result, actual_result);
}

// 1.⁠ ⁠[9,3,5] target is 11 (I returned true)
//  2.⁠ ⁠[11] target 11 ( I returned true)
//  3.⁠ ⁠[10,10,10] target 20 (I return false)
//  4.⁠ ⁠[10, 12,18] target 20 (I return true)
// I was given [9,3,5] to return 11
