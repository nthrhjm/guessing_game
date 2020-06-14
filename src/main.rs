extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101); //1~100までの乱数を生成
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess = String::new(); //空文字列を可変変数guessに代入
    io::stdin()
        .read_line(&mut guess) //標準入力にguessを渡す（標準入力の内容をguessに入力)
        .expect("Failed to read line"); //read_line()が返すResultがErrの場合クラッシュさせる
    println!("You guessed: {}", guess);
}
