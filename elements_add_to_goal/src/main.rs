// Good morning! Here's your coding interview problem for today.

// This problem was recently asked by Google.

// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

// Bonus: Can you do this in one pass?

fn main() {
    let list_of_numbers = [1, 1, 1, 1, 1, 1, -3, 20];
    let goal_value = 17;

    println!("check: {}", two_elements_add_to_goal(&list_of_numbers, goal_value))
}

fn two_elements_add_to_goal(no_list: &[i32], goal: i32) -> bool {
    for (i, no) in no_list.iter().enumerate() {
        for next_no in no_list[i..].iter() {
            if no + next_no == goal {return true}
        }
    }
    false
}
