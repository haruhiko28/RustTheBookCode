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
}
