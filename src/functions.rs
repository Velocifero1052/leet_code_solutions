use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::HashMap;

fn some_func(nums: Vec<i32>, n: i32) -> Vec<i32>{

    let mut i:usize = 0;
    let mut new_vec = Vec::new();

    while i < n as usize {
        new_vec.push(nums[i]);
        new_vec.push(nums[i + n as usize]);
        i += 1;
    }
    return new_vec;
}

fn number_of_good_pairs(nums: Vec<i32>) -> i32 {

    let mut i:usize = 0;
    let mut num_of_pairs:i32 = 0;

    while i < nums.len() - 1 {
        let mut j:usize = i + 1;
        while j < nums.len(){
            if nums[i] == nums[j]{
                num_of_pairs += 1;
            }
            j += 1;
        }
        i += 1;
    }

    return num_of_pairs;
}

fn permutation_build(nums: Vec<i32>) -> Vec<i32> {

    let mut i:usize = 0;
    let mut new_vec:Vec<i32> = Vec::new();

    while i < nums.len(){
        new_vec.push(nums[nums[i] as usize]);
        i += 1
    }

    return new_vec
}

fn count_with_absolute_difference(nums: Vec<i32>, k: i32) -> i32{
    let mut count:i32 = 0;
    let mut i:usize = 0;

    while i < nums.len() - 1{
        let mut j:usize = i + 1;
        while j < nums.len(){
            let diff = nums[i] - nums[j];
            if diff.abs() == k{
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }

    return count;
}

pub(crate) fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32{
    let mut count:i32 = 0;
    let mut i:usize = 0;

    while i < mat.len(){
        if i != mat.len() - i - 1 {
            count += mat[i][i];
            count += mat[i][mat.len() - i - 1];
        }
        else{
            count += mat[i][i];
        }

        i += 1;
    }

    return count;
}

fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {

    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut count:i32 = 0;

    while i < nums.len() - 1 as usize{
        j = i + 1;
        while j < nums.len(){
            if nums[i] == nums[j] && i * j % k as usize == 0{
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }

    return count;
}

fn truncate_sentence(s: String, k: i32) -> String {
    let mut split = s.split(" ");
    let vec: Vec<&str> = split.collect();

    let mut val = "".to_owned();

    let mut i:usize = 0;
    let mut first:bool = true;
    while i < k as usize{
        if first {
            val.push_str(vec[i]);
            first = false;
        }else{
            val.push_str(" ");
            val.push_str(vec[i]);
        }
        i += 1;
    }

    return val;
}

fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool{
    let mut i: usize = 0;
    let mut word_one:String = "".to_owned();
    let mut word_two:String = "".to_owned();

    while i < word1.len(){
        word_one.push_str(word1[i].as_str());
        i += 1;
    }
    i = 0;
    while i < word2.len(){
        word_two.push_str(word2[i].as_str());
        i += 1;
    }

    return word_one == word_two;
}

fn divide_array(nums: Vec<i32>) -> bool{

    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut index_to_skip:HashSet<usize> = HashSet::new();
    let pair_count:usize = nums.len() / 2;
    let mut count: usize = 0;

    while i < nums.len() - 1{
        j = i + 1;
        while j < nums.len(){
            if nums[i] == nums[j] && !index_to_skip.contains(&i) && !index_to_skip.contains(&j){
                count += 1;
                index_to_skip.insert(i);
                index_to_skip.insert(j);
            }
            j += 1;
        }
        i += 1;
    }

    return pair_count == count;
}

fn running_sum(nums: Vec<i32>) -> Vec<i32>{

    let mut sum_vec:Vec<i32> = Vec::new();
    let mut i:usize = 0;

    while i < nums.len(){
        let mut j:usize = 0;
        let mut sum:i32 = 0;
        while j <= i{
            sum += nums[j];
            j += 1;
        }
        sum_vec.push(sum);
        i += 1;
    }

    return sum_vec;
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32{
    let mut max:i32 = 0;
    let mut i:usize = 0;

    while i < accounts.len(){
        let mut sum: i32 = 0;
        let mut j: usize = 0;
        while j < accounts[i].len(){
            sum += accounts[i][j];
            j += 1;
        }
        i += 1;
        if max < sum{
            max = sum;
        }
    }

    return max;
}

fn final_value_after_operations(operations: Vec<String>) -> i32{
    let mut val: i32 = 0;
    let inc:String = String::from("X++");
    let pinc:String = String::from("++X");
    let dec:String = String::from("X--");
    let pdec:String = String::from("--X");

    for op in &operations{
        let str = String::from(op);
        println!("{}", str);
        if str == inc || str == pinc{
            val += 1;
        }else if str == dec || str == pdec{
            val -= 1;
        }else{
            continue;
        }
    }

    return val;
}

fn most_words_found(sentences: Vec<String>) -> i32{
    let mut max = 0;
    let mut i: usize = 0;
    while i < sentences.len(){
        let count = sentences[i].matches(' ').count();
        if count as i32 > max{
            max = count as i32;
        }
        i += 1;
    }
    return max + 1;
}

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool>{

    let mut i:usize = 0;
    let mut new_vec:Vec<bool> = Vec::new();
    let mut max: i32 = 0;

    while i < candies.len(){
        if candies[i] >= max{
            max = candies[i];
        }
        i += 1;
    }

    i = 0;

    while i < candies.len(){
        new_vec.push(candies[i] + extra_candies >= max);
        i += 1;
    }

    return new_vec;
}

fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32>{
    let mut i: usize = 0;
    let mut new_vec:Vec<i32> = Vec::new();

    while i < nums.len(){
        let mut j:usize = 0;
        let mut count:i32 = 0;
        while j < nums.len(){
            if i == j {
                j += 1;
                continue;
            }
            println!("{}", nums[i].to_string().len());
            if nums[i] > nums[j]{
                count += 1;
            }
            j += 1;
        }
        new_vec.push(count);
        i += 1;
    }

    return new_vec;
}

fn find_numbers(nums: Vec<i32>) -> i32{
    let mut count:i32 = 0;
    let mut i: usize = 0;

    while i < nums.len(){
        if nums[i].to_string().len() % 2 == 0{
            count += 1;
        }
        i += 1;
    }

    return count;
}

fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut new_vec:Vec<i32> = Vec::new();
    let mut i:usize = 0;

    while i < index.len(){
        new_vec.insert(index[i] as usize, nums[i]);
        i += 1;
    }

    return new_vec;
}

//2
fn restore_string(s: String, indices: Vec<i32>) -> String{

    let mut str = s.to_owned();
    let mut i:usize = 0;

    while i < indices.len(){
        let index = indices[i] as usize;
        str.remove(index);
        str.insert(index, s.as_bytes()[i] as char);
        i += 1;
    }

    return str;
}

//3
fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let mut count:i32 = 0;
    let mut i:usize = 0;

    while i < words.len(){
        if words[i].starts_with(&pref){
            count += 1;
        }
        i += 1;
    }

    return count;
}

//4
pub fn first_palindrome(words: Vec<String>) -> String {

    let mut i:usize = 0;

    while i < words.len(){
        let mut j: usize = 0;

        if words[i].len() == 1{
            return words[i].to_string();
        }

        while j < words[i].len() / 2{
            if words[i].as_bytes()[j] as char != words[i].as_bytes()[words[i].len() - 1 - j] as char{
                break;
            }

            if j + 1 == words[i].len() / 2{
                return words[i].to_string();
            }

            j += 1;
        }

        i += 1;
    }

    return "".to_string();
}

//5
fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut i:usize = 0;
    let mut data:Vec<(usize, i32)> = Vec::new();


    while i < mat.len(){
        let mut j:usize = 0;
        let mut count:i32 = 0;
        while j < mat[i].len(){
            count += mat[i][j];
            j += 1;
        }
        data.push((i, count));
        i += 1;
    }

    i = 0;

    data.sort_by(|a, b|{
        if a.1 < b.1{
            Ordering::Less
        }else if a.1 == b.1 && a.0 < b.0 {
            Ordering::Less
        }else{
            Ordering::Equal
        }
    });

    i = 0;
    let mut weakest_rows:Vec<i32> = Vec::new();

    while i < k as usize{
        weakest_rows.push(data[i].0 as i32);
        i += 1;
    }

    return weakest_rows;
}

//6
fn repeated_n_times(nums: Vec<i32>) -> i32{
    let mut number:i32 = 0;
    let mut map:HashMap<i32, usize> = HashMap::new();
    let n:usize = nums.len() / 2;
    let mut i:usize = 0;

    while i < nums.len(){
        if !map.contains_key(&nums[i]){
            map.insert(nums[i], 0);
        }

        let value_option = map.get(&nums[i]);
        let mut value = value_option.unwrap();

        if value + 1 == n{
            return nums[i];
        }

        let new_value = value + 1;
        map.insert(nums[i], new_value);
        i += 1;
    }

    return number;
}

//8
fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indexes:Vec<i32> = Vec::new();
    let mut array_to_sort = nums;

    array_to_sort.sort();

    println!("{:?}", array_to_sort);

    let mut i:usize = 0;

    while i < array_to_sort.len(){
        if array_to_sort[i] == target{
            indexes.push(i as i32);
        }

        i += 1;
    }

    return indexes;
}

//9
fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_product:i32 = 0;

    let mut first_max =  0;
    let mut first_max_index = 0;
    let mut i:usize = 0;

    while i < nums.len(){
        if nums[i] > first_max{
            first_max = nums[i];
            first_max_index = i;
        }
        i += 1;
    }

    let mut second_max = 0;
    i = 0;
    while i < nums.len(){
        if nums[i] > second_max && first_max_index != i{
            second_max = nums[i];
        }

        i += 1;
    }
    max_product = (first_max - 1) * (second_max - 1);
    return max_product;
}

//10
fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut min = nums[0];
    let mut max = 0;

    while i < nums.len(){
        if nums[i] < min{
            min = nums[i];
        }
        if nums[i] > max{
            max = nums[i];
        }
        i += 1;
    }

    return gcd(min, max);
}

fn gcd(a: i32, b: i32) -> i32{

    if a == 0{
        return b;
    }

    if b == 0{
        return a;
    }

    if a == b{
        return a;
    }

    if a > b {
        return gcd(a - b, b);
    }
    return gcd(a, b - a);
}

fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut lucky:Vec<i32> = Vec::new();
    let mut i:usize = 0;


    while i < matrix.len(){
        let mut j:usize = 0;

        while j < matrix[i].len(){
            if matrix[i][j] == min_in_row(&matrix[i]){
                println!("{}", matrix[i][j]);
                if matrix[i][j] == max_in_column(&matrix, j){
                    lucky.push(matrix[i][j]);
                    break;
                }
            }
            j += 1;
        }


        i += 1;
    }

    return lucky;
}

fn min_in_row(row: &Vec<i32>) -> i32{
    let mut min_in_row = row[0];
    let mut i = 0;

    while i < row.len(){
        if min_in_row > row[i]{
            min_in_row = row[i];
        }
        i += 1;
    }

    return min_in_row;
}

fn max_in_column(matrix: &Vec<Vec<i32>>, column: usize) -> i32{
    let mut i:usize = 0;
    let mut max_in_column:i32 = 0;

    while i < matrix.len(){
        if matrix[i][column] > max_in_column{
            max_in_column = matrix[i][column]
        }
        i += 1;
    }

    return max_in_column;
}

use std::collections::HashMap;

fn count_good_triplets(nums: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;

    let mut i:usize = 0;

    while i < nums.len() - 3{
        let mut j: usize = i + 1;
        while j < nums.len() - 2{
            let mut k: usize = j + 1;

            while k < nums.len() - 1{
                let mut d: usize = k + 1;

                while d < nums.len(){

                    if nums[i] + nums[j] + nums[k] == nums[d]{
                        count += 1;
                    }

                    d += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    return count;
}

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {

    let mut i: usize = 0;
    let mut new_nums = nums.clone();

    while i < new_nums.len(){
        new_nums[i] = new_nums[i] * new_nums[i];
        i += 1;
    }
    new_nums.sort();

    return new_nums;
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    let mut i:usize = m as usize;

    while i < (m + n) as usize{
        nums1[i] = nums2[i - m as usize];
        i += 1;
    }
    nums1.sort();
}

fn single_number(nums: Vec<i32>) -> i32 {
    let num: i32 = 0;
    let mut i: usize = 0;
    let mut map:HashMap<i32, usize> = HashMap::new();

    while i < nums.len(){

        let opt = map.get_mut(&nums[i]);

        match opt{
            Some(value) => *value += 1,
            None => {map.insert(nums[i], 1);},
        }

        i += 1;
    }

    for (k, v) in map.iter(){
        if *v == 1{
            return *k;
        }
    }

    return 0;
}

fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut i:usize = 0;
    let mut base:i32 = original;

    while i < nums.len(){

        if nums[i] == base{
            base *= 2;
            i = 0;
            continue;
        }

        i += 1;
    }

    return base;
}


fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut i:usize = 1;

    while i < arr.len() - 1{
        if arr[i - 1] < arr[i] && arr[i] > arr[i + 1]{
            return i as i32;
        }
        i += 1;
    }

    return i as i32;
}

fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let mut i:usize = 0;
    let mut count:i32 = 0;

    while i <start_time.len() && i < end_time.len(){
        if start_time[i] <= query_time || query_time <= end_time[i] {
            count += 1;
        }

        i += 1;
    }

    return count;
}

fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut res:Vec<i32> = Vec::new();
    let mut i:usize = 0;

    let mut a = nums1;
    let mut b = nums2;
    let mut c = nums3;

    a.sort();
    b.sort();
    c.sort();


    while i < a.len(){
        let res = b.binary_search(&a[i]);


    }

    return res;
}


pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    if target.len() != arr.len(){
        return false;
    }

    let mut copy1 = target;
    let mut copy2 = arr;
    copy1.sort();
    copy2.sort();

    let mut i:usize = 0;
    while i < copy2.len(){
        if copy1[i] != copy2[i]{
            return false;
        }
    }

    return true;
}
use std::cmp;
fn third_max(nums: Vec<i32>) -> i32 {

    if nums.len() == 1{
        return nums[0];
    }

    if nums.len() == 2{
        return cmp::max(nums[0], nums[1]);
    }

    let mut max:i32 = i32::MIN;
    let mut i:usize = 0;
    let mut max_index:usize = 0;
    while i < nums.len(){
        if nums[i] > max{
            max = nums[i];
            max_index = i;
        }
        i += 1;
    }

    i = 0;
    let mut second_max:i32 = i32::MIN;
    let mut second_max_index:usize = 0;
    let mut found:bool = false;
    while i < nums.len(){
        if nums[i] >= second_max && nums[i] < max && i != max_index{
            second_max = nums[i];
            second_max_index = i;
            found = true;
        }
        i += 1;
    }
    if !found{
        return max;
    }

    i = 0;
    let mut third_max:i32 = i32::MIN;
    found = false;
    while i < nums.len(){
        if nums[i] < second_max && nums[i] < max && i != second_max_index && i != max_index && nums[i] >= third_max{
            third_max = nums[i];
            found = true;
        }
        i += 1;
    }
    if !found{
        return max;
    }

    return third_max;
}