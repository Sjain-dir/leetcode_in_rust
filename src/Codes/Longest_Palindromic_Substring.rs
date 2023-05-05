pub fn checker(maxlen : &mut usize, lo : &mut usize, mut left : usize, mut right : usize , s : &String){
    while left >= 0 && right < s.len() && &s.as_bytes()[left] == &s.as_bytes()[right] {
        println!("maxlen = {} lo = {} left = {} right = {}", *maxlen, *lo, left, right );
        if (right - left + 1) > *maxlen {
            *maxlen = right - left + 1;
            *lo = left ;
        }
        if left == 0 {break;}
        left -= 1;
        right += 1;
    }
}

pub fn longest_palindrome(s: String) -> String {
    let mut lo = 0;
    let mut maxlen: usize = 0;
    if s.len() < 2 {return s;}
    if s.len() == 2 {
        if s.as_bytes()[0] == s.as_bytes()[1] {
            return s
        }
        else {
            return (s.as_bytes()[0] as char).to_string()
        }
    }
    for i in (1 .. s.len()) {
        checker(&mut maxlen, &mut lo, i, i, &s);
        checker(&mut maxlen, &mut lo, i, i+1, &s);
    }
    return s[lo .. (lo + maxlen)].to_string();
}

