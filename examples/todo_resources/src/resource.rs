use rocket_resources::{ RocketResource, NothingInput };

use rocket::request::Form;
use rocket::response::{Flash, Redirect, Failure};
use rocket_contrib::Template;
use rocket::http;
use rocket;

use std;

use super::db;
use super::task::Task;
use super::Context;

pub struct TaskResource<'f> {
    marker: std::marker::PhantomData<&'f ()>,
}

impl<'f> RocketResource for TaskResource<'f> {
    type Id = i32;
    type InputCreate = Form<'f, Task>;
    type InputUpdate = NothingInput;
    type Requirements = db::Conn;

    type CreateResponse = Flash<Redirect>;
    type ReadResponse   = Failure;
    type UpdateResponse = Result<Redirect, Template>;
    type DeleteResponse = Result<Flash<Redirect>, Template>;

    fn create(input: Self::InputCreate, _: http::ContentType, conn: Self::Requirements)
        -> Self::CreateResponse
    {
        let todo = input.into_inner();
        if todo.description.is_empty() {
            Flash::error(Redirect::to("/"), "Description cannot be empty.")
        } else if todo.insert(&conn) {
            Flash::success(Redirect::to("/"), "Todo successfully added.")
        } else {
            Flash::error(Redirect::to("/"), "Whoops! The server failed.")
        }
    }

    fn read(_: Self::Id, _: http::ContentType, _: Self::Requirements)
        -> Self::ReadResponse
    {
        Failure(http::Status::NotImplemented)
    }

    fn update(_: Self::InputUpdate, id: Self::Id, _: http::ContentType, conn: Self::Requirements)
        -> Self::UpdateResponse
    {
        if Task::toggle_with_id(id, &conn) {
            Ok(Redirect::to("/"))
        } else {
            Err(Template::render("index", &Context::err(&conn, "Couldn't toggle task.")))
        }
    }

    fn delete(id: Self::Id, _: http::ContentType, conn: Self::Requirements)
        -> Self::DeleteResponse
    {
        if Task::delete_with_id(id, &conn) {
            Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
        } else {
            Err(Template::render("index", &Context::err(&conn, "Couldn't delete task.")))
        }
    }
}

mod boilerplate {
    use super::TaskResource;
    use rocket;
    use rocket_resources::boilerplate;

    use rocket::http;

    fn create_handler<'r>(r: &'r rocket::Request, data: rocket::Data) -> rocket::handler::Outcome<'r> {
        boilerplate::create_handler::<TaskResource<'r>>(r, data)
    }

    fn read_handler<'r>(r: &'r rocket::Request, d: rocket::Data) -> rocket::handler::Outcome<'r> {
        boilerplate::read_handler::<TaskResource<'r>>(r, d)
    }

    fn update_handler<'r>(r: &'r rocket::Request, d: rocket::Data) -> rocket::handler::Outcome<'r> {
        boilerplate::update_handler::<TaskResource<'r>>(r, d)
    }

    fn delete_handler<'r>(r: &'r rocket::Request, d: rocket::Data) -> rocket::handler::Outcome<'r> {
        boilerplate::delete_handler::<TaskResource<'r>>(r, d)
    }

    pub(super) static
    STATIC_ROUTES_FOR_RESOURCE_TASK : [rocket::StaticRouteInfo; 5] = [
        rocket::StaticRouteInfo {
            method: http::Method::Post,
            path: "/",
            format: None,
            rank: None,
            handler: create_handler,
        },
        rocket::StaticRouteInfo {
            method: http::Method::Patch,
            path: "/<id>",
            format: None,
            rank: None,
            handler: update_handler,
        },
        rocket::StaticRouteInfo {
            method: http::Method::Put,
            path: "/<id>",
            format: None,
            rank: None,
            handler: update_handler,
        },
        rocket::StaticRouteInfo {
            method: http::Method::Delete,
            path: "/<id>",
            format: None,
            rank: None,
            handler: delete_handler,
        },
        rocket::StaticRouteInfo {
            method: http::Method::Get,
            path: "/<id>",
            format: None,
            rank: None,
            handler: read_handler,
        }
    ];
} 

pub fn routes() -> Vec<rocket::Route> {
    boilerplate::STATIC_ROUTES_FOR_RESOURCE_TASK
        .iter()
        .map(From::from)
        .collect::<Vec<rocket::Route>>()
}
