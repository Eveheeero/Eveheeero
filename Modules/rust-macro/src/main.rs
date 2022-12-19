// https://doc.rust-lang.org/reference/macros-by-example.html - 기본적 매크로 및 토큰
// https://doc.rust-lang.org/reference/macros.html - 매크로 문법
// https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros - 함수형 매크로
// https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros - 어트리뷰트 매크로
// https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros - derive에 적용되는 매크로
// https://doc.rust-lang.org/reference/attributes.html - 러스트 기본 어트리뷰트

use rust_macro_core::*;

make_function!(customname);

fn main() {
    say_hello!();
    // say_hello!(true);
    say_hello!(false);
    say_hello!(abcd);

    println!("{}", make_string!());
    customname();
}

#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
    (false) => {
        println!("Not Hello!");
    };
    (abcd) => {
        println!("REAL Hello!");
    };
}
