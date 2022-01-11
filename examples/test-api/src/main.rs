#[macro_use]
extern crate rocket;

use rand::{thread_rng, Rng};
use rocket::{
    http::{ContentType, Status},
    response::status::{BadRequest, NotFound},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/ping")]
fn ping() -> String {
    format!("hello")
}

#[get("/status/<status>")]
fn status(status: u16) -> (Status, (ContentType, &'static str)) {
    let status = Status::from_code(status);
    (status.unwrap(), (ContentType::JSON, "{}"))
}

#[get("/list")]
fn get_list() -> Json<Vec<String>> {
    let amount = thread_rng().gen::<u16>() % 10 + 10;
    let mut res = Vec::with_capacity(amount.into());
    for _ in 0..amount {
        res.push(Uuid::new_v4().to_string());
    }

    Json(res)
}

#[post("/list", data = "<body>")]
fn create_list_item(body: Json<ListItem>) -> Result<Json<ListItem>, BadRequest<&'static str>> {
    if body.id == body.data {
        Err(BadRequest(Some("id cannot equal data")))
    } else {
        Ok(body)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ListItem {
    pub id: String,
    pub data: String,
}

#[get("/list/<id>")]
fn get_list_item_by_id(id: String) -> Result<Json<ListItem>, NotFound<String>> {
    let chance = thread_rng().gen::<usize>() % 100000;
    let exists = chance > 1;
    if exists {
        let item = ListItem {
            id: id.clone(),
            data: Uuid::new_v4().to_string(),
        };
        Ok(Json(item))
    } else {
        Err(NotFound(format!(
            "item {} not found. Chance: {}",
            id, chance
        )))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            hello,
            ping,
            status,
            get_list,
            get_list_item_by_id,
            create_list_item
        ],
    )
}
