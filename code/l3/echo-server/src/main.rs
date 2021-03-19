use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Echo Server Starting...");

    // 设置监听地址
    let ip_addr = "127.0.0.1:8888";

    // let listener = TcpListener::bind(&ip_addr).unwrap();

    // 匹配错误，如果端口被占用会友好提示
    let listener = match TcpListener::bind(&ip_addr) {
        // 可以正常监听，返回listener（ struct `TcpListener`）
        Ok(listener) => listener,
        // 如果端口被占用，调用panic！，并提示错误然后退出
        Err(e) => {
            panic!("Failed to listen @ {}, Error: {}", ip_addr, e);
        }
    };

    // 遍历端口的输入流
    for stream in listener.incoming() {
        // unwrap方法，如果Result是Ok则返回Ok中的值，如果为Err则调用panic！
        let stream = stream.unwrap();

        // 处理stream
        handle_connection(stream);
        // match stream {
        //     Ok(tcp_stream) => {
        //         handle_connection(tcp_stream);
        //     }
        //     Err(e) => {
        //         panic!("Get Error: {}", e);
        //     }
        // }
    }
}


fn handle_connection(mut stream: TcpStream) {
    // 设置一个buff，1024byte
    let mut buffer = [0; 1024];

    // 读取stream，并将内容放到buffer，简写出错
    stream.read(&mut buffer).unwrap();

    // 格式化字符串，将buffer中的内容转成string并赋值给response
    let response = format!("Echo: {}", String::from_utf8_lossy(&buffer[..]));

    // 将response回显给客户端
    stream.write(response.as_bytes()).unwrap();

    // 在屏幕打印获取的值
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}