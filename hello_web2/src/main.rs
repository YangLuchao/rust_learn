// src/main.rs
use hello_web2::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // 监听7878端口
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 构建线程池
    let pool: ThreadPool = ThreadPool::new(4);

    // 拿到两个迭代器
    for stream in listener.incoming().take(2) {
        // 拿到stream中的内容
        let stream: TcpStream = stream.unwrap();
        // 放入到线程池中执行
        pool.execute(
            // 闭包参数，这一整块都是闭包
            || {
                handle_connection(stream);
            },
        );
    }

    println!("Shutting down.");
}

/// 处理链接函数
fn handle_connection(mut stream: TcpStream) {
    // 构建个缓冲区
    let mut buffer = [0; 1024];
    // 将数据读到缓冲区里
    stream.read(&mut buffer).unwrap();

    // 解析资源匹配
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // get请求，返回hello.html
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // get sleep请求，线程睡5秒后再返回hello.html
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        // 没有找到就返回404
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 找到要返回的文件
    let contents: String = fs::read_to_string(filename).unwrap();

    // 返回文件格式化
    let response: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // 返回
    stream.write_all(response.as_bytes()).unwrap();
    // 刷新
    stream.flush().unwrap();
}
