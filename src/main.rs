use std::any::type_name;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
lalrpop_mod!(pub calculator2); // synthesized by LALRPOP
lalrpop_mod!(pub calculator3); // synthesized by LALRPOP
lalrpop_mod!(pub calculator4);

pub mod ast;

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // println!("Hello, world!");
    // println!("{}", calculator2::TermParser::new().parse("(22)").unwrap());
    println!(
        "{}",
        calculator3::ExprParser::new()
            .parse("(1 + 2) + (22 - 1) * 3")
            .unwrap()
    );

    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();

    print_type_of(&expr)
}
