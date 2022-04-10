use std::cmp::{max, min};

pub struct Solution {
    inputs: Vec<(Vec<i32>, Vec<i32>)>
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        if  len1 > len2 {
            Solution::find_median_sorted_arrays(nums2, nums1)
        } else {
            let mut left  = 0;
            let mut right = len1;

            let mut left_max  = 0;
            let mut right_min = 0;

            while left <= right {
                let i: usize = (left + right) / 2;
                let j: usize = (len1 + len2 + 1) / 2 - i;

                let nums_li = if i == 0 { i32::MIN } else { nums1[i - 1] };
                let nums_ri = if i == len1 { i32::MAX } else { nums1[i] };
                let nums_lj = if j == 0 { i32::MIN } else { nums2[j- 1] };
                let nums_rj = if j == len2 { i32::MAX } else { nums2[j] };

                if nums_li <= nums_rj {
                    left_max = max(nums_li, nums_lj);
                    right_min = min(nums_ri, nums_rj);
                    left = i + 1;
                } else {
                    right = i - 1;
                }
            }
            let left_f = left_max as f64;
            let right_f = right_min as f64;
            if (len1 + len2) % 2 == 0 { (left_f + right_f) / 2.0 } else { left_f }
        }
    }
}

fn main() {
    let sln = Solution {
        inputs: vec![(vec![1, 2], vec![3]), (vec![1,2,4], vec![1,3,5]),
                     (vec![1, 2], vec![3,4])],
    };

    for input in sln.inputs {
        print!("input: ");
        print!("[");
        for x in &input.0 {
            print!(" {}", x)
        }
        print!(" ], ");
        print!("[");
        for x in &input.1 {
            print!(" {}", x)
        }
        println!(" ]");

        let output = Solution::find_median_sorted_arrays(input.0, input.1);

        println!("output: {}", output);
    }
}
