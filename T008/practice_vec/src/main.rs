fn main() {
    let v = vec![1,2,3];
    // 初期値のi32値を与えたので、コンパイラはvの型がVec<i32>であると推論できた

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    {
        let v = vec![2,3,4];
    }

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];

    println!("The third element is {}.",third);

    match v.get(2) {
        Some(third) => println!("The third element is {}.",third),
        None => println!("There is no third element!"),
    }

    let does_not_exist = &v[100];
    let does_not_exist = &v.get(100);
    // .get()だとpanicにならず、Noneが返ってくる→Some/Noneが優秀

    let mut v = vec![1,2,3,4,5];

    let first = &mut v[0];

    v.push(6);

    println!("The first element is: {}", first);
}
