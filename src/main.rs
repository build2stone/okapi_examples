use rocket::serde::Serialize;
use rocket::{get, FromForm};
use rocket_okapi::{openapi, openapi_spec, settings::OpenApiSettings, JsonSchema};

#[derive(Serialize, FromForm, JsonSchema)]
#[schemars(example = "example_param")]
struct Param(i32);

fn example_param() -> Param {
    Param(10)
}

#[openapi]
#[get("/example?<param>")]
fn get_param(param: Param) {}

fn main() {
    let spec = openapi_spec![get_param](&OpenApiSettings::default());
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());
}
