# 如何写一个简单echo服务

## 前置知识
1. socket
2. tcp
3. 字节流
4. `read`-`write`
5. 阻塞I/O
6. 线程

## 需要使用的类库

* std::thread
* std::net::{TcpListener,TcpStream,Shutdown}
* std::io::{Read,Write}
