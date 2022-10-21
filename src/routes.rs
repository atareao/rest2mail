use actix_web::{get, post, Error, HttpResponse, http::header::ContentType,
    HttpRequest};
use serde::Serialize;
use serde_json::{Value, json};
use std::env;
use crate::{mail::Mail, mailer::Mailer};

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    content: Value,
}
impl Respuesta {
    fn new(code: i32, content: Value) -> Result<HttpResponse, Error>{
        let respuesta = Respuesta{
            code,
            status: if code < 300 {"OK".to_string()} else {"KO".to_string()},
            content,
        };
        match code{
            0 ..= 299 => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
            _ => Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
        }
    }

    fn simple(code: i32, message: &str) -> Result<HttpResponse, Error>{
        Respuesta::new(code, json!({"description": message}))
    }

}

#[get("/")]
pub async fn root(req: HttpRequest) -> Result<HttpResponse, Error> {
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key("Authorization") || 
            req.headers().get("Authorization").unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
    Respuesta::simple(200, "Up and running")
}

#[post("/send")]
pub async fn send(req: HttpRequest, post: String) -> Result<HttpResponse, Error> {
    let server = env::var("SERVER").expect("SERVER not set");
    let username = env::var("SERVER_USERNAME").expect("SERVER_USERNAME not set");
    let password = env::var("SERVER_PASSWORD").expect("SERVER_PASSWORD not set");
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key("Authorization") || 
            req.headers().get("Authorization").unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
    let mut post_content: Value = serde_json::from_str(&post).unwrap();
    let from = match post_content.get_mut("from") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, 'from' is mandatory")
    };

    let to = match post_content.get_mut("to") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, 'to' is mandatory")
    };
    let subject = match post_content.get_mut("subject") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, 'subject' is mandatory")
    };
    let body = match post_content.get_mut("body") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, 'body' is mandatory")
    };
    let mail = Mail::new(&from, &to, &subject, &body);
    let mailer = Mailer::new(&server, &username, &password);
    match mailer.send(mail){
        Ok(_) => Respuesta::simple(200, "Send"),
        Err(e) => Respuesta::simple(500, &e.to_string()),
    }
}
