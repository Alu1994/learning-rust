fn main() {
    // ===============================================================
    // ====================== primitive data types ===================
    // ===============================================================

    // ==============================
    // ====== scalar data types =====
    // ==============================
    
    // ==== integers ====
    let x: i32 = 2;
    // i8
    // i16
    // i32
    // i64
    // i128

    // unsigned integer *non negative*
    let x: u32 = 2;
    // u8
    // u16
    // u32
    // u64
    // u128

    // u8 0 ~ 2^8 -1 = 0 ~ 255
    // i8 -2^7 ~ 2^7 -1 = -128 ~ 127


    let x: u8 = 4;
    let y = x;
    println!("{}, {}", x, y);


    // ==== floating points ====
    // there is only 2 floating points the *f64* is the default one
    let x: f32 = 10.9;
    let x: f64 = 20.2;

    // ==== boolean ====
    let true_or_false: bool = false;
    let true_or_false: bool = true;

    // ==== char ====
    let letter: char = 'a';


    // ==============================
    // ==== compound data types =====
    // ==============================

    // ==== tuple ====
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let mut tup2: (i8, bool, char) = (2, false, 't');
    tup2.0 = 10;
    println!("{}", tup2.0);

    // ==== arrays ====
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);
}