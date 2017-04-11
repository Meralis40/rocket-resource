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
    use rocket_resources::RocketResource;

    use rocket::data::FromData;
    use rocket::request::FromRequest;
    use rocket::response::Responder;
    use rocket::outcome::Outcome::*;
    use rocket::http;

    fn create_handler<'r>(
        request: &'r rocket::Request,
        data: rocket::Data
    ) -> rocket::handler::Outcome<'r> {
        let requirements = match <<TaskResource<'r> as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let input = match <<TaskResource<'r> as RocketResource>::InputCreate as FromData>::from_data(request, data) {
            Success(val) => val,
            Forward(data) => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let response = <TaskResource<'r> as RocketResource>::create(input, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    fn update_handler<'r>(
        request: &'r rocket::Request,
        data: rocket::Data
    ) -> rocket::handler::Outcome<'r> {
        let requirements = match <<TaskResource<'r> as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let id = match request.get_param::<<TaskResource<'r> as RocketResource>::Id>(0) {
            Ok(x) => x,
            Err(_) => return Failure(http::Status::BadRequest),
        };

        let input = match <<TaskResource<'r> as RocketResource>::InputUpdate as FromData>::from_data(request, data) {
            Success(val) => val,
            Forward(data) => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let response = <TaskResource<'r> as RocketResource>::update(input, id, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    fn delete_handler<'r>(
        request: &'r rocket::Request,
        data: rocket::Data
    ) -> rocket::handler::Outcome<'r> {
        let requirements = match <<TaskResource<'r> as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let id = match request.get_param::<<TaskResource<'r> as RocketResource>::Id>(0) {
            Ok(x) => x,
            Err(_) => return Failure(http::Status::BadRequest),
        };

        let response = <TaskResource<'r> as RocketResource>::delete(id, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    pub(super) static
    STATIC_ROUTES_FOR_RESOURCE_TASK : [rocket::StaticRouteInfo; 4] = [
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
    ];
} 

pub fn routes() -> Vec<rocket::Route> {
    boilerplate::STATIC_ROUTES_FOR_RESOURCE_TASK
        .iter()
        .map(From::from)
        .collect::<Vec<rocket::Route>>()
}
