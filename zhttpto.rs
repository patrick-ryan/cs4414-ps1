//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut VISITOR_COUNT: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            let pa = match request_str.find_str("/") {
                Some(x) => x,
                None => -1,
            };
            let th = match request_str.find_str(" HTTP") {
                Some(x) => x,
                None => -1,
            };
            
            if pa == -1 || th == -1 {
                println!("Error parsing request");
            }
            else if (pa+1) == th {
                unsafe { VISITOR_COUNT += 1 };
    
                let response: ~str = 
                    ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                     </style></head>
                     <body>
                     <h1>Greetings, Krusty!</h1>" +
                     format!("<h2>Number of visits: {:d}</h2>", unsafe { VISITOR_COUNT }) +
                     "</body></html>\r\n";
                stream.write(response.as_bytes());
            }
            else {
                if request_str.find_str(".html") == None {
                    let forbidden: ~str = 
                        ~"HTTP/1.1 418 I'm a teapot\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body { background-color: #111; color: #FFEEAA }
                                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                         </style></head>
                         <body>
                         <h1>Error 418</h1>
                         This server only accepts .html files!
                         </body></html>\r\n";
                    stream.write(forbidden.as_bytes());
                }
                else {
                    match File::open(&Path::new(request_str.slice(pa+1, th))) {
                        Some(mut file) => {
                            let contents = 
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n".as_bytes() +
                                file.read_to_end();
                            stream.write(contents);
                        },
                        None => println!("Error reading file"),
                    }
                }
            }
            println!("Connection terminates.");
        }
    }
}

