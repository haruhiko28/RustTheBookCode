fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", world");
        println!("{}", r1);
        println!("{}", s);

    }
 // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
    let r3 = &s;
    let r4 = &s;
    // let r2 = &mut s;    
    println!("{}",r3);
    // println!("{}",r2);

}
