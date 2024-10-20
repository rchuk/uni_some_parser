use uni_some_parser::list_parser;

pub fn main() {
    let res = list_parser::list("[1,1,2,3,5,8]");
    println!("Result: {res:?}");
    assert_eq!(res, Ok(vec![1, 1, 2, 3, 5, 8]));
}
