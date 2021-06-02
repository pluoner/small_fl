// return a new sorted merged list from K sorted lists, each with size N.

// For example, if we had [[10, 15, 30], [12, 15, 20], [17, 20, 32]], the result should be [10, 12, 15, 15, 17, 20, 20, 30, 32]

fn main() {
    let a_list_of_sorted_lists = [[10, 15, 30], [12, 15, 20], [17, 20, 32]];

    println!("results: {:?}", combine_sorted_lists(&a_list_of_sorted_lists))
}

fn combine_sorted_lists<SortedLists: AsRef<[List]>, List: AsRef<[i32]>> (sorted_list: SortedLists) -> Vec<i32> {
    let mut result_list: Vec<i32> = Vec::new();
    vec!(1, 2, 3)
}
