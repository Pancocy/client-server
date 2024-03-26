/* 解析请求体模块 */

use std::collections::HashMap;

use crate::httprequest::Resource;

//创建：方法、版本、资源的枚举;http请求体的结构体
#[derive(PartialEq,Debug)]
pub enum Methods{
    Get,
    Post,
    Uninitialized
}
#[derive(PartialEq,Debug)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized
}
pub enum Source{
    Path(String)
}
pub struct HttpRequest{
    pub method : Methods,
    pub version: Version,
    pub header: HashMap<String,String>,
    pub resource:Resource,
    pub body : String
}

//为这些枚举实现from trait
impl From<&str> for Methods{
    fn from(value: &str) -> Self {
        match value {
            "GET" => Methods::Get,
            "POST" => Methods::Post,
            _ => Methods::Uninitialized
        }
    }
}

impl From<&str> for Version{
    fn from(value: &str) -> Self {
        match value { 
            "HTTP/1.1" => Version::V1_1,
            "HTTP/1.2" => Version::V2_0,
            _ => Version::Uninitialized
        }
    }
}

impl From<String> for HttpRequest{
    fn from(value: String) -> Self {
        let mut expect_method = Methods::Uninitialized;
        let mut expect_version = Version::Uninitialized;
        let mut expect_resource = Resource::Path("".to_string());
        let mut expect_header = HashMap::new();
        let mut expect_body = "".to_string();
        for line in value.lines(){
            if line.contains("HTTP") {
                let (method,version,resource) = process_request_line(line);
                expect_method = method;
                expect_version = version;
                expect_resource = resource;
            }else if line.contains(":") {
                let (key,value) = process_header_line(line);
                expect_header.insert(key,value);
            }else {
                expect_body = line.to_string()
            }
        }
        HttpRequest{
            method:expect_method,
            version:expect_version,
            resource:expect_resource,
            header:expect_header,
            body:expect_body
        }
    }
}

//处理解析方法行
pub fn process_request_line(line:&str) -> (Methods,Version,Resource){
    let mut trim_line = line.split_whitespace();
    let  method =  trim_line.next().unwrap();
    let version = trim_line.next().unwrap();
    let resource = trim_line.next().unwrap();
    (
        method.into(),
        version.into(),
        //resource没有实现from trait
        Resource::Path(resource.to_string())
    )
}

//处理header行的方法
pub fn process_header_line(line:&str) -> (String,String){

    let mut split_line = line.split(":");
    let mut k = String::from("");
    let mut v = String::from("");

    if let Some(key) = split_line.next(){
        k = key.to_string();
    }
    if let Some(value) = split_line.next(){
        v =  value.to_string();
    }
    (k,v)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_methods_version(){
        assert_eq!(Methods::Get,"GET".into());
        assert_eq!(Methods::Post,"POST".into());
    }

    #[test]
    fn test_process_line(){
        let test_line = "GET HTTP/1.1 /greeting";
        let line = process_request_line(test_line);
        assert_eq!(line,("GET".into(),"HTTP/1.1".into(),Resource::Path("/greeting".to_string())));
    }

    #[test]
    fn test_process_header(){
        let test_header = String::from("Content-type:Text/html\r\nAccept:*\r\nUser-Agent:Chrome 84");
        let x:HttpRequest = test_header.into();

        let mut expect_header = HashMap::new();

        expect_header.insert("Content-type".to_string(),"Text/html".to_string());
        expect_header.insert("Accept".to_string(),"*".to_string());
        expect_header.insert("User-Agent".to_string(),"Chrome 84".to_string());

        assert_eq!(x.header,expect_header)

    }
}