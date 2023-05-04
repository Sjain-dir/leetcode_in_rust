fn scope( i : usize, length1 : usize, nums1: &Vec<i32>, j : usize, length2 : usize, nums2: &Vec<i32>) -> bool {
    if j < length2{

        if i < length1 {
            if &nums1[i] <= &nums2[j] {true} else {false}
        }
        else {
            false
        }
    }
    else {
        true
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length1 = nums1.len() ;
    let length2 = nums2.len() ;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let total_length = length1+length2;
    if total_length%2 != 0 {
        let mut median = (total_length +1)/2;
        loop {
            if scope(i, length1, &nums1, j, length2, &nums2) {
                i += 1;
                median -= 1;
                //println!("i = {} median = {}" , i, median);
                if median == 0{
                    return nums1[i-1] as f64;
                }
            }
            else {
                j += 1;
                median -= 1;
                //println!("j = {} median = {}" , i, median);
                if median == 0{
                    return nums2[j-1] as f64;
                }
            }
        }
    }
    else {
        let mut median1 = (total_length as i32)/2;
        let mut median2 = ((total_length as i32)/2) + 1;
        // to do median2 as median1 but at -1 
        let mut median1_value: i32 = 0;
        let mut median2_value: i32 = 0;

        loop {
            if scope(i, length1, &nums1, j, length2, &nums2) {
                i += 1;
                median1 -= 1;
                median2 -= 1;
               // println!("i = {} median1 = {} median2 = {}" , i, median1, median2);
                if median1 == 0 {
                    median1_value = nums1[i-1];
                }
                if median2 == 0 {
                    median2_value = nums1[i-1];
                    break;
                }
               // println!("in i median1 = {} median2 = {}" ,median1_value, median2_value);
            }
            else {
                j += 1;
                median1 -= 1;
                median2 -= 1;
                //println!("j = {} median1 = {} median2 = {}" , j, median1, median2);
                if median1 == 0 {
                    median1_value = nums2[j-1];
                }
                if median2 == 0 {
                    median2_value = nums2[j-1];
                   // println!("num2 = {}" ,nums2[j] );
                    break;
                }
              //  println!("in j median1 = {} median2 = {}" ,median1_value, median2_value);
            }
        }
     //   println!("median1 = {} median2 = {}" ,median1_value, median2_value);
        return (median1_value as f64 + median2_value as f64 )/2.0;


    }
}