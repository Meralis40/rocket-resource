extern crate rocket;

use rocket::request;
use rocket::http;
use rocket::data;
use rocket::response;

pub trait RocketResource {
    type Id: for<'a> request::FromParam<'a>;
    type Requirements: for<'a, 'r> request::FromRequest<'a, 'r>;
    type CreateResponse: for<'r> response::Responder<'r>;
    type ReadResponse: for<'r> response::Responder<'r>;
    type UpdateResponse: for<'r> response::Responder<'r>;
    type DeleteResponse: for<'r> response::Responder<'r>;
    type Input: data::FromData;

    fn create(input: Self::Input, format: http::ContentType, requirements: Self::Requirements)
        -> Self::CreateResponse;

    fn read(id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::ReadResponse;

    fn update(input: Self::Input, id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::UpdateResponse;

    fn delete(id: Self::Id, format: http::ContentType, requirements: Self::Requirements)
        -> Self::DeleteResponse;
}

