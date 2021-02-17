use diesel::prelude::*;

fn main() {
    use hello::schema::{comments, posts};

    let conn = hello::establish_connection();

    let query = posts::table
        .left_join(comments::table)
        .select(posts::title)
        .filter(comments::id.is_null());
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    let data: Vec<String> = query.load(&conn).unwrap();

    dbg!(data);
}
