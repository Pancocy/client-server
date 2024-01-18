use std::collections::HashMap;
#[warn(dead_code)]
//定义请求方法的枚举,并且需要为它实现三个trait：From,
#[derive(Debug,PartialEq)]
pub enum Methods{
    Get,
    Post,
    Uninitialized
}

impl From<&str> for Methods{
    fn from(s: &str) -> Methods {
        match s {
            "GET"   => Methods::Get,
            "POST" => Methods::Post,
            _ => Methods::Uninitialized
        }
    }
}

//定义请求http版本的枚举
#[derive(Debug,PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}
impl From<&str> for Version{
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Resource{
    Path(String)
}

//定义请求结构体
pub struct HttpRequest{
    pub method : Methods,
    pub version: Version,
    pub headers: HashMap<String,String>,
    pub resource: Resource,
    pub body:String
}

impl From<String> for HttpRequest{
    fn from(value: String) -> Self {
        let mut parsed_method = Methods::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_header = HashMap::new();
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_body = "";

        for line in value.lines() {
            if line.contains("HTTP"){
                let (method,version,resource) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            }else if line.contains(":") {
                let (key,value) = process_header_line(line);
                parsed_header.insert(key,value);
            }else {
                parsed_body = line;
            }
        }
        HttpRequest{
            method:parsed_method,
            version:parsed_version,
            headers:parsed_header,
            resource:parsed_resource,
            body:parsed_body.to_string()
        }
    }
}

fn process_req_line(line: &str) -> (Methods,Version,Resource) {
    let mut line  = line.split_whitespace();

    //这的顺序非常重要！！
    let  method = line.next().unwrap();
    let  resource = line.next().unwrap();
    let  version = line.next().unwrap();

    (
        method.into(),
        version.into(),
        Resource::Path(resource.to_string())
    )

}

fn process_header_line (line:&str) -> (String,String) {
        let mut line = line.split(":");
        let mut key  = String::from("");
        let mut value = String::from("");

        if let Some(k) = line.next(){
            key = k.to_string();
        }
        if let Some(v) = line.next(){
            value = v.to_string();
        }
        (key,value)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_method_work(){
        let x : Methods = "GET".into();
        assert_eq!(Methods::Get,x)
    }

    #[test]
    fn test_version_work(){
        let x :Version = "HTTP/1.1".into();
        assert_eq!(Version::V1_1,x)
    }

    #[test]
    fn test_process_line(){
        let  line:String = String::from("GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:Mozilla/5.0\r\nAccept:*\r\n\r\n");

        let mut expected_header =  HashMap::new();
        expected_header.insert("Host".into(),"localhost".into());
        expected_header.insert("User-Agent".into(),"Mozilla/5.0".into());
        expected_header.insert("Accept".into(),"*".into());

        let value:HttpRequest =  line.into();

        assert_eq!(Methods::Get,value.method);
        assert_eq!(Version::V1_1,value.version);
        assert_eq!(Resource::Path("/greeting".to_string()),value.resource);
        assert_eq!(expected_header,value.headers);

    }

}



