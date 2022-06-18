

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
   let mut min_in_row = 0;
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


fn main() {

   let mut matrix:Vec<Vec<i32>> = Vec::new();
   let a = vec![3,7,8];
   let b = vec![9,11,13];
   let c = vec![15,16,17];
   matrix.push(a);
   matrix.push(b);
   matrix.push(c);

   println!("{:?}", lucky_numbers(matrix));
}
