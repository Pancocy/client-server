
use httplib::httprequest::{HttpRequest, Resource};
use httplib::httpresponse::HttpResponse;
use super::handler::{StaticHandler, NotFoundHandler, WebServiceHandler, Handler};
use std::io::{Write};

pub struct Router;

impl Router{
    pub fn route(req:HttpRequest,stream:&mut impl Write) -> (){
        match &req.method {
            httplib::httprequest::Methods::Get =>   match req.resource {
                Resource::Path(ref s) => {
                    let route:Vec<&str> = s.split("/").collect();
                    println!("{:?}",route);
                    match route[1] {
                        "api" => {
                            let resp:HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send(stream);
                        }
                        _ => {
                            let resp:HttpResponse = StaticHandler::handle(&req);
                            let _ = resp.send(stream);
                        }
                    }
                }
            },
            _ =>{
                let  resp:HttpResponse = NotFoundHandler::handle(&req);
                let _ = resp.send(stream);
            }
        }
    }
}