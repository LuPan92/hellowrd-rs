fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5/3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z'

    let tupA = (1,2,3);
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    
    let a[i32;5] = [1,2,3,4,5];
}
