use std::thread;
use std::net::{TcpListener,TcpStream,Shutdown};
use std::io::{Read,Write};

// 所有的unwrap()都是我们假定前面的Optional[T] must be Some(T)
// 处理被接受的字节流
fn handle_connection(mut stream: TcpStream) {
    // 以0x0填充一个长度为1KB的字节buffer
    let mut data = [0 as u8; 1024];
    // 这里的while match很有意思，正常来说我们写while(condition) {
    //     while-body
    // }
    // 我们在下面case种的condition非常长，而body是empty pass
    while match stream.read(&mut data) {
       Ok(size) => {
        // 将读入的字节流原样写入响应流,
        // 注意这里忽略了写入响应流的字节数返回.unwrap()的返回值是usize(即写入的字节数)
        stream.write(&data[0..size]).unwrap();
        // 为什么要return true？因为，我要在读取size的字节后，继续进行读取任务
        true
       },
       Err(err) => {
           println!("An error occurred, terminating connection with {}",stream.peer_addr().unwrap());
           println!("Error is {}",err);
           // 关闭字节流（输入和输出都关闭)
           stream.shutdown(Shutdown::Both).unwrap();
            // 无它，跳出request的处理
           false
       }
    } {
        // 我才是真正的while-body
    }
}

fn main() {
    // 注意，这里是一个包装后的简写。
    // 如果你使用C-like API，则这里至少应该出现3行代码
    // 以下为了简化代码，使用Ruby的描述(Ruby的C socket binding)
    // (1) 创建一个IPV4的tcp字节流;
    // socket = Socket::new(socket::INET,socket::STREAM)
    // (2) 绑定;
    // addr = Socket.pack_sockaddr_in(4481,'0.0.0.0')
    // (3) 侦听;
    // socket.bind(addr)
    let listener = TcpListener::bind("0.0.0.0:3030").unwrap();
    // 监听端口3030
    println!("Server listening on port {}",3030);
    // 轮询接受的字节流(请求)
    for stream in listener.incoming() {
        // 这里的match用来区分无错误的字节流（请求）与发生错误（比如，最大连接数或者超时）
        match stream {
            Ok(stream) => {
                println!("New connection: {}",stream.peer_addr().unwrap());
                // fork一个新的thread进行连接处理
                // 为什么要fork一个新的？
                // 如果不这样会不会导致阻塞？！
                thread::spawn(move || {
                    handle_connection(stream)
                });
            }
            Err(e) => {
                // 打印捕获的错误                
                println!("Error : {}",e);
            }
        }
    }
    // 关闭打开的socket服务
    // https://doc.rust-lang.org/rust-by-example/trait/drop.html
    // Box, Vec, String, File, and Process are some examples of types 
    // that implement the Drop trait to free resources. 
    // The Drop trait can also be manually implemented for any custom data type.
    // 你可以认为这是一个类似析构函数的东西，它会帮你把file、socket这些资源
    // 清理干净
    drop(listener);
}