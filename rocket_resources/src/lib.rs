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

