### Rust Web全栈
1. #### client客户端

2. #### server服务端
       server服务端需要引用http模块，http模块中包含两个请求和响应模块

3. #### httpServer:用于声明http请求的路由模块、handler模块等、依赖httplib中的模块
   1. handler模块：
      > 1. 定义handler trait,实现handl方法、load_file加载文件方法。
      > 2. 定义处理三类不同路由的结构体（api:web请求、“static”静态资源请求、Not Found请求）、定义orderStatus用于web请求的订单状态
      > 3. 为web请求实现load_json方法，用于返回响应的body
   2. 路由模块：
      > 1. 定义路由结构体，用于声明路由
      > 2. 实现路由结构体，实现new方法、route方法、route方法match接收httpRequest的Resource的path路径来分配三类路由
   3. server模块：
      > 1. 定义server结构体，用于声明server
      > 2. 实现server结构体的new方法、run方法、在run方法中实现对host建立并对每一次请求进行监听，
4. #### httplib:用于声明http响应体&请求体的模块
   1. http响应体：
      ```makefile
      http请求报文格式:
      GET /index.html HTTP/1.1     //METHOD GREETING HTTP_VERSION

      Host: www.rust-lang.org       //HEASDER LINES
      User-Agent: curl/7.64.1
      Accept: */*

      "hello,world"                 //BODY
        ```
   2. http请求体：
      解析http请求报文