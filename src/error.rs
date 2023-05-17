use actix_web::{
    HttpRequest,
    HttpResponse,
};

/* HTTP Error used in PaperMoon
 - NotFound
 - InternalServerError
 */

async fn handle_error(err: actix_web::Error, _: &HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    // Handle specific errors or error types if needed
    // For example, you can check for specific status codes and return custom responses

    // Return a generic error response with a 500 status code
    Ok(HttpResponse::InternalServerError().body("Internal Server Error"))
}
