#![Display]
fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6,3,2];
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        fn non_decreasing(integers :&mut Vec<i32>) {
            let mut sorted: Vec<i32> = Vec::new;
            let mut buffer: Vec<i32> = Vec::new;
            let mut non_decreasing_buffer_length: i32 = 0;
            let mut vec_length = integers.len();
            let mut prev_int = i32;
            let mut unsorted: Vec<i32> = Vec::new;
            unsorted = integers;

            while i < vec_length {
            for i in integers {
                match i {
                < prev_int => {
                    buffer.push(i);
                    prev_int = i;
                    non_decreasing_buffer_length +=1;
                }
                >= prev_int => {
                    unsorted.push(i)
                }
                None => break;
            }
            }

            
            }
            println!("{}{}{}{}{}{}", sorted,to_string(), buffer,to_string(), non_decreasing_buffer_length.to_string(), vec_length.to_string(), prev_int.to_string(), unsorted.to_string());
        }
    }
}