use actix_web::{
    dev::ServiceResponse,
    http::header::{ContentType, HeaderValue},
    middleware::ErrorHandlerResponse,
    HttpResponseBuilder,
    Result,
};



pub fn render_error_page<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let status = res.status();
    let (request, response) = res.into_parts();
    let def_msg = HeaderValue::from_static("");
    let msg = response.headers().get("X-Err-Msg")
        .unwrap_or(&def_msg)
        .to_str()
        .unwrap();
    let html = format!("
        <!DOCTYPE html>
        <head>
            <title> Error ! - PaperMoon</title>
            <link rel=\"shortcut icon\" type=\"image/x-icon\" href=\"/misc/favicon.ico\"/>
            <link rel=\"stylesheet\" href=\"/static/style.css\"/>
            <style>
                body {{
                    background-color: #282828;
                    margin: 0;
                    padding: 0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    text-align: center;
                }}

                .status-code {{
                    font-size: 5rem;
                    font-weight: bold;
                    color: #CC241D;
                    margin-bottom: 1rem;
                }}

                .error-message {{
                    font-size: 2rem;
                    color: #869D6A;
                }}
            </style>
        </head>
        <body>
            <div class=\"content\">
                <div class=\"status-code\">{}</div>
                <class=\"error-message\">{}</div>
            </div>
        </body>
    ", status, msg);

    let new_response = HttpResponseBuilder::new(status)
    .insert_header(ContentType::html())
    .body(html);

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
