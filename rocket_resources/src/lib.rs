#![feature(never_type)]

extern crate rocket;

use rocket::request;
use rocket::http;
use rocket::data;
use rocket::response;

pub trait RocketResource {
    type Id: for<'a> request::FromParam<'a>;
    type Requirements: for<'a, 'r> request::FromRequest<'a, 'r>;
    type CreateResponse: response::Responder<'static>;
    type ReadResponse: response::Responder<'static>;
    type UpdateResponse: response::Responder<'static>;
    type DeleteResponse: response::Responder<'static>;
    type InputCreate: data::FromData;
    type InputUpdate: data::FromData;

    /// POST <resource rel>/
    fn create(input: Self::InputCreate, format: http::ContentType, requirements: Self::Requirements)
        -> Self::CreateResponse;

    /// GET <resource rel>/<id>
    fn read(id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::ReadResponse;

    /// PATCH <resource rel>/<id>
    fn update(input: Self::InputUpdate, id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::UpdateResponse;

    /// DELETE <resource rel>/<id>
    fn delete(id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::DeleteResponse;
}

pub struct NothingInput;

impl data::FromData for NothingInput {
    type Error = !;

    fn from_data(_: &rocket::Request, _: rocket::Data) -> data::Outcome<Self, !> {
        rocket::outcome::Outcome::Success(NothingInput)
    }
}

pub mod boilerplate {
    use super::RocketResource;
    use rocket;

    use rocket::data::FromData;
    use rocket::request::FromRequest;
    use rocket::response::Responder;
    use rocket::outcome::Outcome::*;
    use rocket::http;

    pub fn create_handler<'r, T: RocketResource>(request: &'r rocket::Request, data: rocket::Data) 
        -> rocket::handler::Outcome<'r>
    {
        let requirements = match <<T as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val)         => val,
            Forward(_)           => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let input = match <<T as RocketResource>::InputCreate as FromData>::from_data(request, data) {
            Success(val) => val,
            Forward(data) => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let response = <T as RocketResource>::create(input, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    pub fn read_handler<'r, T: RocketResource>(request: &'r rocket::Request, data: rocket::Data)
        -> rocket::handler::Outcome<'r>
    {
        let requirements = match <<T as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let id = match request.get_param::<<T as RocketResource>::Id>(0) {
            Ok(x) => x,
            Err(_) => return Failure(http::Status::BadRequest),
        };

        let response = <T as RocketResource>::read(id, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    pub fn update_handler<'r, T: RocketResource>(request: &'r rocket::Request, data: rocket::Data) 
        -> rocket::handler::Outcome<'r> 
    {
        let requirements = match <<T as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let id = match request.get_param::<<T as RocketResource>::Id>(0) {
            Ok(x) => x,
            Err(_) => return Failure(http::Status::BadRequest),
        };

        let input = match <<T as RocketResource>::InputUpdate as FromData>::from_data(request, data) {
            Success(val) => val,
            Forward(data) => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let response = <T as RocketResource>::update(input, id, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }

    pub fn delete_handler<'r, T: RocketResource>(request: &'r rocket::Request, data: rocket::Data) 
        -> rocket::handler::Outcome<'r> 
    {
        let requirements = match <<T as RocketResource>::Requirements as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let format = match <http::ContentType as FromRequest>::from_request(request) {
            Success(val) => val,
            Forward(_)   => return Forward(data),
            Failure((status, _)) => return Failure(status),
        };

        let id = match request.get_param::<<T as RocketResource>::Id>(0) {
            Ok(x) => x,
            Err(_) => return Failure(http::Status::BadRequest),
        };

        let response = <T as RocketResource>::delete(id, format, requirements);

        match response.respond() {
            Ok(x) => Success(x),
            Err(y) => Failure(y),
        }
    }
}