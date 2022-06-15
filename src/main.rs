

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


fn main() {
   println!("{}", find_numbers(vec![12,345,2,6,7896]));
}
