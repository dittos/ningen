extern crate lalrpop_util;

mod ast;
mod parse;

fn main() {
    println!("{:?}", parse::parse_Document("
type Blog {
    name: String;
    recent_post: Post;
}

type Post {
    title: String;
}
"));
}
