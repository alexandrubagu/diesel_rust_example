use self::models::*;
use diesel::prelude::*;
use diesel_rust_example::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(status.eq("active"))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.email);
        println!("-----------\n");
    }
}
