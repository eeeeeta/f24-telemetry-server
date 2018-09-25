#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate rocket_contrib;
extern crate dotenv;
extern crate chrono;
extern crate failure;

mod schema;
mod pool;
mod models;
mod proto;

use rocket::fairing::AdHoc;
use rocket_contrib::Json;
use proto::{UploadBody, AuthDetails};
use rocket::response::status::Custom;
use rocket::http::Status;
use pool::DbConn;
use models::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[post("/r0/data/<car_id>/upload?<auth_details>", data = "<body>")]
fn upload_data(c: DbConn, car_id: String, auth_details: AuthDetails, body: Json<UploadBody>) -> Result<Custom<&'static str>, ::failure::Error> {
    let token: Option<AccessToken> = {
        use schema::access_tokens::dsl::*;
        access_tokens.filter(token.eq(&auth_details.access_token))
            .first(&*c)
            .optional()?
    };
    let token = match token {
        Some(t) => t,
        None => return Ok(Custom(Status::Unauthorized, "Invalid access token."))
    };
    if token.disabled {
        return Ok(Custom(Status::Unauthorized, "Access token disabled."));
    }
    let team: Team = {
        use schema::teams::dsl::*;
        teams.filter(id.eq(&token.team_id))
            .first(&*c)?
    };
    if team.current_race.is_none() {
        return Ok(Custom(Status::BadRequest, "Your team isn't racing at the moment."));
    }
    let car: Option<Car> = {
        use schema::cars::dsl::*;
        cars.filter(identifier.eq(&car_id).and(team_id.eq(&team.id)))
            .first(&*c)
            .optional()?
    };
    let car = match car {
        Some(t) => t,
        None => return Ok(Custom(Status::BadRequest, "Invalid car; it either doesn't exist, or you don't have permissions for it."))
    };
    let ts = match NaiveDateTime::from_timestamp_opt(body.0.ts as _, 0) {
        Some(t) => t,
        None => return Ok(Custom(Status::BadRequest, "Invalid timestamp."))
    };
    let data = DataRecord {
        car_id: car.id,
        race_id: team.current_race.unwrap(),
        ts,
        ..body.0.as_data_record()
    };
    {
        use schema::data_records;

        diesel::insert_into(data_records::table)
            .values(&data)
            .execute(&*c)?;
    }
    Ok(Custom(Status::Ok, "Data uploaded."))
}

#[get("/")]
fn root() -> String {
    format!("{} version {} up and running!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
fn main() {
    rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            info!("Setting up database connection...");
            Ok(pool::attach_db(rocket))
        }))
        .mount("/", routes![root, upload_data])
        .launch();
}
