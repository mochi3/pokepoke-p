extern crate pokepoke_rust;
extern crate diesel;

use self::pokepoke_rust::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use pokepoke_rust::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(id.eq(3))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
        println!("{}", post.title);
    }
}