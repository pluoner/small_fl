/*
There's a staircase with N steps, and you can climb 1 or 2 steps at a time.
Given N, write a function that returns the number of unique ways you can climb the staircase. The order of the steps matters.

For example, if N is 4, then there are 5 unique ways:

1, 1, 1, 1
2, 1, 1
1, 2, 1
1, 1, 2
2, 2
What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number from a set of positive integers X?
For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time. Generalize your function to take in X.
 */
fn main() {

    let no_stairs = 25;
    let allowed_steps = vec!(1, 3, 5);

    let mut result: Vec<Vec<i32>> = Vec::new();
    all_combos(no_stairs, &allowed_steps, Vec::new(), &mut result);
    println!("{:?}", result);
}

fn all_combos(remaining_stairs: i32, step_list: &Vec<i32>, current_combo: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    for step in step_list.iter() {
        let mut next_combo = current_combo.clone();
        if remaining_stairs - step == 0 {
            next_combo.push(*step);
            &result.push(next_combo.clone());
        } else if remaining_stairs - step > 0 {
            next_combo.push(*step);
            all_combos(remaining_stairs - step, step_list, next_combo, result);
        }
    }
}
