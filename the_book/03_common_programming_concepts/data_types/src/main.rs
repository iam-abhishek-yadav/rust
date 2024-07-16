fn main() {

//Integers

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    let a = 98_222; //Decimal
    let b = 0xff; //Hex
    let c = 0o77; //Octal
    let d = 0b1111_0000; //Binary
    let e = b'A'; //Byte (u8 only)


//Floating-point numbers

    let f = 2.0; // f64
    let g: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


//Boolean

    let h = true;
    let i: bool = false;


//Character
    let j = 'z';
    let k: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


//Tuples

    let tup = ("rust", 100_000);
    let (name, count) = tup;
    let count = tup.1;


//Array

    let error_codes = [200, 404, 500];
    let nor_found = error_codes[1];
    let x = error_codes[3];

    let byte = [0; 8];
}
