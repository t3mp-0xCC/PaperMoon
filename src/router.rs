use actix_web::{
    get,
    Error,
    HttpResponse,
    HttpRequest,
};
use tera::Tera;
use log::debug;

use crate::{cruds, error};
use crate::error::error_page_response;

/* index */
#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("index"))
}


/* article */
#[get("/article")]
async fn article_list() -> Result<HttpResponse, Error> {
    /* Tera */
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Template")),
    };
    let mut tera_ctx = tera::Context::new();
    let posts = match cruds::get_post_list(){
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Failed to get posts list from DB")),
    };
    tera_ctx.insert("posts", &posts);
    let view = match tera.render("article_list.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Rendering")),
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/article/{article_id}")]
async fn article(req: HttpRequest) -> Result<HttpResponse, Error> {
    let matcher = req.match_info();
    let article_id = match matcher.get("article_id") {
        Some(id) => id,
        None => return Ok(error_page_response(404, "article_id is missing")),
    };
    debug!("article_id: {:?}", article_id);
    let article = match cruds::get_post_from_content_id(&article_id.to_string()) {
        Ok(a) => a,
        Err(_) => return Ok(error_page_response(404, &format!("{} is invalid article_id", article_id))),
    };
    debug!("article found: {}", article.title);

    /* Tera */
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(_) => return Ok(error_page_response(500, "Something went wrong in Tera Template")),
    };

    let mut tera_ctx = tera::Context::new();
    // key, value
    tera_ctx.insert("title", &article.title);
    tera_ctx.insert("article", &article.content_html);
    let view = match tera.render("article.html.tera", &tera_ctx) {
        Ok(v) => v,
        Err(_) => return Ok(error_page_response(500, "Something wrong in Tera rendering")),

    };

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

/* about */
#[get("/about")]
async fn about() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("about"))
}

/* portfolio */
#[get("/portfolio")]
async fn portfolio() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("portfolio"))
}

/* for hecker */
#[get("/..%2F..%2F..%2F/etc/passwd")]
async fn joke() -> Result<HttpResponse, Error> {
    let passwd = String::from("
    root:x:0:0:root:/root:/bin/bash</br>
    daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin</br>
    bin:x:2:2:bin:/bin:/usr/sbin/nologin</br>
    sys:x:3:3:sys:/dev:/usr/sbin/nologin</br>
    sync:x:4:65534:sync:/bin:/bin/sync</br>
    games:x:5:60:games:/usr/games:/usr/sbin/nologin</br>
    man:x:6:12:man:/var/cache/man:/usr/sbin/nologin</br>
    lp:x:7:7:lp:/var/spool/lpd:/usr/sbin/nologin</br>
    mail:x:8:8:mail:/var/mail:/usr/sbin/nologin</br>
    news:x:9:9:news:/var/spool/news:/usr/sbin/nologin</br>
    uucp:x:10:10:uucp:/var/spool/uucp:/usr/sbin/nologin</br>
    proxy:x:13:13:proxy:/bin:/usr/sbin/nologin</br>
    www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin</br>
    backup:x:34:34:backup:/var/backups:/usr/sbin/nologin</br>
    list:x:38:38:Mailing List Manager:/var/list:/usr/sbin/nologin</br>
    irc:x:39:39:ircd:/run/ircd:/usr/sbin/nologin</br>
    gnats:x:41:41:Gnats Bug-Reporting System (admin):/var/lib/gnats:/usr/sbin/nologin</br>
    nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin</br>
    _apt:x:100:65534::/nonexistent:/usr/sbin/nologin</br>
    systemd-network:x:101:102:systemd Network Management,,,:/run/systemd:/usr/sbin/nologin</br>
    systemd-resolve:x:102:103:systemd Resolver,,,:/run/systemd:/usr/sbin/nologin</br>
    messagebus:x:103:104::/nonexistent:/usr/sbin/nologin</br>
    systemd-timesync:x:104:105:systemd Time Synchronization,,,:/run/systemd:/usr/sbin/nologin</br>
    rtkit:x:105:111:RealtimeKit,,,:/proc:/usr/sbin/nologin</br>
    sshd:x:106:65534::/run/sshd:/usr/sbin/nologin</br>
    t3mp:0w0:1000:1000:t3mp:/home/t3mp:/bin/bash</br>
    chronos-access:x:1001:1001:chronos-access:/dev/null:/bin/false</br>
    android-everybody:x:665357:665357:android-everybody:/dev/null:/bin/false</br>
    android-root:x:655360:655360:android-root:/dev/null:/bin/false</br>
    systemd-coredump:x:999:999:systemd Core Dumper:/:/usr/sbin/nologin</br>
    polkitd:x:998:998:polkit:/nonexistent:/usr/sbin/nologin</br>
    usbmux:x:107:46:usbmux daemon,,,:/var/lib/usbmux:/usr/sbin/nologin</br>
    ");
    Ok(HttpResponse::Ok().content_type("text/html").body(passwd))
}
