 struct Solution
 
 impl Solution {
   fn produce_except_self(s: Vec<i32>) -> Vec<i32> {
     let mut result: Vec<i32> = Vec::new();
     for _ in nums {
       result.push(1);
     }
     let n = nums.len();
     let mut left = 1;
     let mut right = 1;
     
     for i in 0..nums.len() {
       result.get_mut(i)  
       result[i] *= left;
       left = result[i];
       result[n-1-i] *= right;
       right = result[n-1-i];
     }
     result
   }
 }