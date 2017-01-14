extern crate lalrpop_util;

mod ast;
mod parse;

fn main() {
    println!("{:?}", parse::parse_Document("
type ID = Int;
type Timestamp = Long;

struct User {
    id: ID;
    name: String;
    date_joined: Timestamp;
    is_twitter_connected?: Bool;
    categories?: [Category];
}

struct Category {
    id: ID;
    name: String;
}

enum StatusType {
    FINISHED;
    WATCHING;
    SUSPENDED;
    INTERESTED;
}

struct Record {
    id: ID;
    user_id: ID;
    work_id: ID;
    category_id?: ID;
    title: String;
    status: String;
    status_type: StatusType;
    updated_at: Timestamp;
    has_newer_episode?: Bool;
    user?: User;
}

struct Post {
    id: ID;
    user_id: ID;
    record_id: ID;
    status: String;
    status_type: StatusType;
    comment: String;
    updated_at: Timestamp;
    contains_spoiler: Bool;
    record?: Record;
    user?: User;
}

struct Work {
    id: ID;
    title: String;
    image_url?: String;
    alt_titles?: [String];
    episodes?: [Episode];
    record_count: Int;
    rank?: Int;
    record?: Record;
    metadata?: WorkMetadata;
}

struct Episode {
    number: Int;
    post_count?: Int;
}

struct WorkMetadata {
    title: String;
    links: WorkLinks;
    studios?: [String];
    source?: WorkSource;
    schedule: {String: WorkSchedule};
}

struct WorkLinks {
    website?: String;
    namu?: String;
    ann?: String;
}

enum WorkSource {
    MANGA;
    ORIGINAL;
    LIGHT_NOVEL;
    GAME;
    FOUR_KOMA;
    VISUAL_NOVEL;
    NOVEL;
}

struct WorkSchedule {

}
"));
}
