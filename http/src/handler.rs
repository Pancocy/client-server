use std::{env, fs};
use std::collections::HashMap;
use httplib::httprequest::HttpRequest;
use httplib::httpresponse::HttpResponse;

use serde::{Serialize, Deserialize};
use serde_json;


pub trait Handler {
    fn handle(req:&HttpRequest) -> HttpResponse;
    fn load_file(path:&str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, path);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}
pub struct NotFoundHandler;
pub struct StaticHandler;
pub struct WebServiceHandler;


#[derive(Serialize,Deserialize)]
pub struct OrderStatus{
    order_id : i32,
    order_date: String,
    order_status:String
}

impl Handler for NotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new(
            404,
            None,
            Some(Self::load_file("404.html").unwrap()),
        )
    }
}
impl Handler for StaticHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let httplib::httprequest::Resource::Path(s) = &req.resource;
        let route:Vec<&str>  = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new(200, None, Self::load_file("index.html")),
            "about" => HttpResponse::new(200, None, Self::load_file("about.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map = HashMap::new();
                    if path.ends_with(".js") {
                        map.insert("Content-type", "text/javascript");
                    } else if path.ends_with(".css") {
                        map.insert( "Content-type", "text/css");
                    } else {
                        map.insert("Content-type", "text/html");
                    }
                    HttpResponse::new(200, Some(map), Some(contents))
                },
                None => HttpResponse::new(404, None, Self::load_file("404.html"))
            }
        }
    }
}
impl WebServiceHandler{
    fn load_json() -> Vec<OrderStatus>{
        let default_path = format!("{}/data",env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        let json_path = format!("{}/{}",data_path,"data.json");
        let json_content = fs::read_to_string(json_path);
        let order:Vec<OrderStatus> = serde_json::from_str(json_content.unwrap().as_str()).unwrap();
        order
    }
}

impl Handler for WebServiceHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let httplib::httprequest::Resource::Path(s) = &req.resource;
        let route:Vec<&str> = s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "order" => {
                let mut map = HashMap::new();
                map.insert("content-type","application/json");
                HttpResponse::new(200,Some(map),Some(serde_json::to_string(&Self::load_json()).unwrap()))
            },
            _ => HttpResponse::new(404,None,Self::load_file("404.html"))
        }
    }
}



