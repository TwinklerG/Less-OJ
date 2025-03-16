#[macro_use]
extern crate rocket;

mod controller;
mod service;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            controller::c_controller,
            controller::cpp_controller,
            controller::c_test_controller,
            controller::cpp_test_controller,
            controller::java_test_controller,
            controller::python3_test_controller,
            controller::go_test_controller
        ],
    )
}
