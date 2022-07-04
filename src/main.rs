mod functions;

fn is_strictly_increasing(nums: &Vec<i32>) -> bool{
   let mut i:usize = 0;

   while i < nums.len() - 1{
      if nums[i] >= nums[i + 1]{
         return false;
      }
   }
   return true;
}


fn can_be_increasing(nums: Vec<i32>) -> bool {

   if nums.len() == 2{
      return nums[0] < nums[1];
   }

   if is_strictly_increasing(&nums){
      return true;
   }

   let mut i:usize = 1;

   while i < nums.len() - 1{

   }

   return false;
}

fn main() {

   let mut a = vec![1,2,-2147483648];

   println!("{}", third_max(a));

}
