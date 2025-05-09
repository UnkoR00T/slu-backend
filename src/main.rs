#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![create_jwt, protected_route])
}
