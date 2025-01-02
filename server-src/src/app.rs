use rocket::launch;

#[launch]
fn rocket_launch() -> _ {
    rocket::build()
}