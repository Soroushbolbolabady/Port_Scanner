use std::net::TcpStream;
use std::net::SocketAddr;
use std::time::Duration;




fn main() {
    

    println!("Welcome to Port Scanner!");


    let mut ports: [i32;1024] = [0; 1024];

    for i in 1..1024{
        ports[i] = i as i32;
    }

    for port in ports{

        let address: SocketAddr = format!("56.228.31.38:{}", port.to_string()).parse().unwrap();
        
        let timeout = Duration::from_millis(70);
        let result = TcpStream::connect_timeout(&address, timeout);
       

        match result {
            Ok(_stream) => {
                println!("This address is Open: {}", &address)
            }
            Err(_e)=>{
                continue;
            }
            
        }
    }



} 
