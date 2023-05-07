pub fn convert(s: String, num_rows: i32) -> String{
    let num_rows = num_rows as usize;
    let mut result = String::new();
    let mut temp : char;
    let mut ref_index : usize;
    for i in 0..(num_rows-1) {
        let jump = 2*num_rows-2;
        ref_index = i;
        loop {
            if i == 0 || i == num_rows-1 {
                temp = match s.chars().nth(ref_index) {
                    Some(t) => t,
                    None => break,
                };
                print!("{} {}. ", ref_index ,s.chars().nth(ref_index).expect(""));
                result.push(temp);
                ref_index += jump;
            }
            else {
                temp = match s.chars().nth(ref_index) {
                    Some(t) => t,
                    None => break,
                };
                print!("{} {}. ", ref_index ,s.chars().nth(ref_index).expect(""));
                result.push(temp);
                temp = match s.chars().nth(ref_index + jump - i*2) {
                    Some(t) => t,
                    None => break,
                };
                print!("{} {}. ", ref_index + jump - i*2 ,s.chars().nth(ref_index + jump - i*2).expect(""));
                result.push(temp);

                ref_index += jump;
            }

        }
    }

    result
}
