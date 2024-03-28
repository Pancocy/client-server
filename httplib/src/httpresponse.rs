//构建http响应体的struct
use std::collections::HashMap;
use std::io::Write;

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
            "{} {} {}\r\n{}\r\n{}",
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
    //实现new放方法
    pub fn new(
        status_code: u16,
        headers: Option<HashMap<&'a str,&'a str>>,
        body: Option<String>,
    ) -> HttpResponse {
            let mut response = HttpResponse::default();
            if status_code != 200 {
                response.status_code = status_code
            };
            response.reason_phrase = match response.status_code {
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
    //实现send方法
    pub fn send(&self, stream: &mut impl Write) -> Result<(), ()>{
        let response = self.clone();
        let res_string = String::from(response);
        println!("response:\r\n{}",res_string);
        let _ = write!(stream, "{}", res_string);
        Ok(())
    }
    //返回响应体各个部分的方法
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
    use super::*;

    #[test]
    fn test_new_response_404(){
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
    fn test_new_response_200(){
        let response = HttpResponse::new(
            200,
            None,
            Some("Hello, World!".to_string()),
        );
        assert_eq!(response.version(),"HTTP/1.1");
        assert_eq!(response.status_code(),200);
        assert_eq!(response.reason_phrase(),"OK");
        assert_eq!(response.headers(),"Content-Type: text/html\r\n");
        assert_eq!(response.body(),"Hello, World!");
    }

    #[test]
    fn test_response_to_string(){
        let response = HttpResponse::new(
            404,
            None,
            Some("Hello, World!".to_string()),
        );
        let response_string:String= response.into();
        assert_eq!(response_string,"HTTP/1.1\r\n404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 13\r\nHello, World!")
    }
}






