use std::collections::HashMap;

//创建解析响应体的结构体
#[derive(Debug,PartialEq,Clone)]
pub struct  HttpResponse <'a> {
        version:&'a str,
        status_code:u16,
        status_text:&'a str,
        header:Option<HashMap<&'a str,&'a str>>,
        body:Option<String>
}

//实现default trait
impl <'a> Default for HttpResponse<'a>{
        fn default() -> Self {
                HttpResponse{
                        version: "HTTP/1.1",
                        status_code:200,
                        status_text:"OK",
                        header:None,
                        body:None
                }
        }
}
//实现from trait
impl <'a> From<HttpResponse<'a>> for String{
        fn from(value: HttpResponse<'a>) -> String {
                let respnse = value.clone();
                format!(
                        "{}\r\n{} {}\r\n{}{}",
                        respnse.version(),
                        respnse.status_code(),
                        respnse.status_text(),
                        respnse.header(),
                        respnse.body()
                )
        }
}

impl <'a> HttpResponse<'a> {
        //实现new方法
        pub fn new(
                status_code:u16,
                header:Option<HashMap<&'a str,&'a str>>,
                body:Option<String>
        ) -> HttpResponse{
                let mut default = HttpResponse::default();
                if status_code != 200 {
                        default.status_code = status_code
                }
                default.status_text = match default.status_code {
                        200 => "OK",
                        404 => "Not Found",
                        500 => "Internal Error",
                        _ => "Some Wrong"
                };
                default.header = match &header {
                        Some(_h) => header,
                        None =>{
                                let mut x   = HashMap::new();
                                x.insert("Content-type","text/html");
                                Some(x)
                        }
                };
                default.body = body;
                default
        }
        //实现send 方法


        //实现getter方法
        fn version(&self) -> &str{
                self.version
        }
        fn status_code(&self) -> u16{
                self.status_code
        }
        fn status_text(&self) -> &str{
                self.status_text
        }
        fn header(&self) -> String{
                let header = self.header.clone().unwrap();
                let mut header_string = String::from("").into();
                for (k,v) in header.iter(){
                        header_string = format!("{}{}:{}\r\n",header_string,k,v)
                }
                header_string
        }
        fn body(&self) -> &str {
                match &self.body {
                        Some(h) => h.as_str(),
                        None => ""
                }
        }
}

#[cfg(test)]

mod test{
        use super::*;

        #[test]
        fn test(){
                /// # 该测试对HttpResponse的new方法、为HttpResponse实现的"trait"
                let expect_response = HttpResponse::new(
                        200,
                        None,
                        Some("xxzxka".to_string())
                );

                let exact_string = String::from("HTTP/1.1\r\n200 OK\r\nContent-type:text/html\r\nxxzxka");
                let expect_string:String = expect_response.into();

                assert_eq!(exact_string,expect_string)
        }


}