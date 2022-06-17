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