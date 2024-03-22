//构建http响应体的struct
use std::collections::HashMap;
use std::io::{Read, Write};
use std::str;

/*为其实现三个trait*/
#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: u16,
    reason_phrase: &'a str,
    headers: Option<HashMap<&'a str,&'a str>>,
    body: Option<String>,
}
//实现default trait
impl <'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: "HTTP/1.1",
            status_code: 200,
            reason_phrase: "OK",
            headers: None,
            body: None,
        }
    }
}

impl <'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse<'a>) -> String {
        let result = response.clone();
        format!(
            "{} \r\n, {} \r\n,{} \r\n,{} \r\n,{}",
            result.version(),
            result.status_code(),
            result.reason_phrase(),
            result.headers(),
            result.body(),
        )
    }
}

//实现new 方法
impl <'a> HttpResponse<'a> {
    pub fn new(
        status_code: u16,
        headers: Option<HashMap<&'a str,&'a str>>,
        body: Option<String>,
    ) -> HttpResponse {
            let mut response = HttpResponse::default();
            if status_code != 200 {
                response.status_code = status_code
                };
            response.reason_phrase = match status_code {
                200 => "OK",
                404 => "Not Found",
                500 => "Internal Server Error",
                501 => "Not Implemented",
                303 => "See Other",
                _ => "Unknown",
            };
            response.headers = match &headers {
                Some(_h) => headers,
                None => {
                    let mut h = HashMap::new();
                    h.insert("Content-Type", "text/html");
                    Some(h)
                }
            };
            response.body = body;
            response
        }
    }
//实现send方法
impl <'a> HttpResponse<'a> {
        pub fn send(&self, stream: &mut (impl Write + Read)) -> String {
            let res = self.clone();
            let res_string = String::from(res);
                stream.write(res_string.as_bytes()).unwrap();
                let mut buffer = [0,128];
                stream.read(&mut buffer).unwrap();
                let rest = str::from_utf8(&buffer).unwrap();
                rest;
        }
        fn version(&self) -> &str {
            self.version
        }
        fn status_code(&self) -> u16 {
            self.status_code
        }
        fn reason_phrase(&self) -> &str {
            self.reason_phrase
        }
        pub fn headers(&self) -> String {
            let map = self.headers.clone().unwrap();
            let mut header_string = String::from("").into();
            for (key, value) in map.iter() {
                header_string = format!("{}{}: {}\r\n", header_string, key, value);
            };
            header_string
        }
        pub fn body(&self) -> &str {
            match &self.body {
                Some(b) => b.as_str(),
                None => "",
            }
        }
    }


#[cfg(test)]
mod test{
    use std::net::TcpStream;
    use std::str;

    use super::*;

    #[test]
    fn test_new_response(){
        let response = HttpResponse::new(
            404,
            None,
            Some("Hello, World!".to_string()),
        );
        assert_eq!(response.version(),"HTTP/1.1");
        assert_eq!(response.status_code(),404);
        assert_eq!(response.reason_phrase(),"Not Found");
        assert_eq!(response.headers(),"Content-Type: text/html\r\n");
        assert_eq!(response.body(),"Hello, World!");
    }

    #[test]
    fn test_send_response(){
        let mut stream = TcpStream::connect("localhost:3000").unwrap();
        let response = HttpResponse::new(
            404,
            None,
            Some("Hello, World!".to_string()),
        );
        let result = response.send(&mut stream).unwrap();

        assert_eq!(str::from_utf8(response).unwrap(),result)


    }
}






