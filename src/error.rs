use actix_web::{
    dev::ServiceResponse,
    http::header::ContentType,
    middleware::ErrorHandlerResponse,
    HttpResponseBuilder,
    Result,
};

/* HTTP Error used in PaperMoon
 - NotFound
 - InternalServerError
 */

pub fn render_error_page<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let status = res.status();
    let request = res.into_parts().0;
    let html = "<h1> Error 0w0 </h1>";

    let new_response = HttpResponseBuilder::new(status)
    .insert_header(ContentType::html())
    .body(html);

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
