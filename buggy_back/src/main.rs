#[macro_use]
extern crate rocket;

#[get("/ticket")]
fn get_test() -> &'static str {
    "This is a get response for /"
}

#[post("/ticket")]
fn post_test() -> &'static str {
    "This is a post response for /"
}

#[delete("/ticket")]
fn delete_test() -> &'static str {
    "This is a delete test for /"
}

#[patch("/ticket")]
fn patch_test() -> &'static str {
    "This is a patch_test for /"
}

#[put("/ticket")]
fn put_test() -> &'static str {
    "This is a put_test for /"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![get_test, post_test, delete_test, patch_test, put_test],
    )
}
