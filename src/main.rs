mod Codes;
use Codes::Zigzag_Conversion;
fn main() {
    let s = String::from("PAYPALISHIRING");
    println!( "Answer is {} " ,  Zigzag_Conversion::convert(s, 3));
}
