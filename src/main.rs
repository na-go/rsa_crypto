use std::io;
use std::str::FromStr;

// エラトステネスの篩で素数判定
fn prime_discriminator(p: usize, q:usize) -> bool {
    // 入力の平方根までが判断に必要
    // 平方根を用意
    /*
    let root_p = (p as f64).sqrt();
    let root_q = (p as f64).sqrt();
    */
    // 上の値までfor文を回して素数か判定したいため整数型にしたい
    // 出来てない
    // let round_p:i64 = (root_p as u64).round();
    // p,qが素数であるかのフラグ
    /*
    let mut p_prime:bool = true;
    let mut q_prime:bool = true;
    */
    println!("ppp:{}, qqq:{}", p,q);
    let prime:bool = {
        // ここに篩
        true
    };
    prime
}


fn input_number() -> (usize, usize){
    // 入力を受け取るところ
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let mut it = buf.split_whitespace().map(|n| usize::from_str(n).unwrap());
    let (p, q) = (it.next().unwrap(), it.next().unwrap());

    // 入力が素数か判定
    let prime_number = prime_discriminator(p, q);
    // 素数であればそのままp,qを返す
    if prime_number{
        return (p, q);
    };
    // 素数でないならp,qとして1,1を返す
    (1, 1)
}


fn main() {
    let (p,q) = input_number();
    println!("p:{},q:{}", p, q);
}
