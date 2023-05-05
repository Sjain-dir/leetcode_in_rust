mod Codes;
use Codes::Longest_Palindromic_Substring;
fn main() {
    let s = String::from("b");
    println!( "Answer is {} " ,  Longest_Palindromic_Substring::longest_palindrome(s));
}
