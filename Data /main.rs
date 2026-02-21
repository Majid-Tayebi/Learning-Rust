fn main() {
    // -- Scalar Types --

    // Integer types
    let a: i8 = 12; // 8-bit - signed integer (-128 to 127)
    let b: u8 = 200; // 8-bit - unsigned integer (0 to 255)

    let c: i16 = 3356; // 16-bit - signed integer (-32,768 to 32,767)
    let d: u16 = 44125; // 16-bit - unsigned integer (0 to 65,535)

    let e: i32 = 554_242_542; // 32-bit - signed integer (-2,147,483,648 to 2,147,483,647)   ==> You can spilt 3 digit with _ for clean code
    let f: u32 = 653_543_353; // 32-bit - unsigned integer (0 to 4,294,967,295)    ==> You can spilt 3 digit with _ for clean code

    let g: i64 = 700_000_000_000; // 64-bit - signed integer (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    let h: u64 = 800_255_659_589; // 64-bit - unsigned integer (0 to 18,446,744,073,709,551,615)

    let i: i128 = 965_695_4785_000; // 128-bit - signed integer (-170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727)
    let j: u128 = 10_000_000_000_000; // 128-bit - unsigned integer (0 to 340,282,366,920,938,463,463,374,607,431,768,211,455)

    let k: isize = 11; // arch - signed integer
    let l: usize = 12; // arch - unsigned integer

    /*
       arch
       depend on the architecture of the computer your program is running on.
       64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    */

   
    // Floating-Point Types

    let n: f32 = 14.0;

    let o: f64 = 15.0;

    let p = 16.0;

    // The Boolean Type

    let q: bool = true;
    let r: bool = false;

    let s = true;

    // The Character Type

    let t: char = 'a';

    let u = 'b';

    let v = '😻';  //Rust Supported Unicode character

    // String Types

    let w: &str = "hello";
    let x: String = String::from("hello");
    let y: String = "hello".to_owned();

    let z = "hello";

    // -- Compound Types --
    // ...
}
