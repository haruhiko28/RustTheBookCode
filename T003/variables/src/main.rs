use std::io;

fn main() {
    // -------------------------------------------------------
    // 変数と可変性
    // -------------------------------------------------------

    // let x = 5;
    // println!("The value of x is: {}", x); // xの値は{}です
    // x = 6;
    // println!("The value of x is: {}", x); // xの値は{}です

    // mutキーワードが使われると、xが束縛している値を5から6に変更
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x); // xの値は{}です

    // 定数は、プログラムが走る期間、定義されたスコープ内でずっと有効    
    // const MAX_POINTS: u32 = 100_000;    

    let x = 5;

    let x = x + 1; // 再定義

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x); // xの値は{}です

    let spaces = "    ";
    let spaces = spaces.len(); 

    // let mut spaces = "   ";
    // spaces = spaces.len(); 
    // spaces = spaces; 

    // -------------------------------------------------------
    // データ型
    // -------------------------------------------------------
    // let guess = "42".parse().expect("Not a Number!"); // 型を指定していないERROR
    let guess: u32 = "42".parse().expect("Not a Number!");

    let x = 2.0; // f64    

    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2/3;
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '🐱';

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 配列
    let a = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5];
    let a = [3; 5];  // let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];

    println!("Please enter an array index."); // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim() // 文字列の先頭と末尾から空白文字を取り除くために使用
        .parse()
        .expect("Index entered was not a number");
    // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is {}",
        index, element
    );
}
