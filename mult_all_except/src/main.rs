// Good morning! Here's your coding interview problem for today.

// This problem was asked by Uber.

// Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.

// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].

// Follow-up: what if you can't use division?

fn main() {
    let a_list = [1, 2, 3, 4, 5];

    println!("first try: {:?}", mult_all_with_div(&a_list));

    println!("Second try: {:?}", mult_all_without_div(&a_list));
    
}
//NB This function depends on all elements in argumentlist being non-zero.
fn mult_all_with_div (input_list: &[i32]) -> Vec<i32> {
    let tot_product = mult_all(&input_list);
    let mut result_list = vec!();
    for i in input_list.iter() {
        result_list.push(&tot_product / i);
    }
    result_list
}

fn mult_all(input_list: &[i32]) -> i32 {
    let mut product = 1;
    for i in input_list.iter() {
        product = product * i;
    }
    product
}
//this one does work with a zero element, though is slow (O(n^2)?)
fn mult_all_without_div (input_list: &[i32]) -> Vec<i32> {
    let mut result_list = vec!();
    for (i, _num) in input_list.iter().enumerate() {
        let mut product: i32 = 1;
        for (j, num) in input_list.iter().enumerate() {
            if i != j {
                product = product * num;
            }
        }
        result_list.push(product);
    }
    result_list
}
//In all I would rather remake the first and make sure to handle the zero element scenario somehow. 