use std::i64;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::time::Duration;
use indicatif::ProgressBar;
use std::io;


// TODO: 1_Try for better output style
//          2_ Add Different mode (important ports, custom ports ,...)
//          3_ Multi Thread 
//



fn main() {
    


    println!(r#"
 ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
 █                                                      █
 █  ░██████╗░█████╗░░█████╗░██╗░░██╗███╗░░░███╗███████╗ █
 █  ██╔════╝██╔══██╗██╔══██╗██║░░██║████╗░████║██╔════╝ █
 █  ╚█████╗░███████║██║░░╚═╝███████║██╔████╔██║█████╗░░ █
 █  ░╚═══██╗██╔══██║██║░░██╗██╔══██║██║╚██╔╝██║██╔══╝░░ █
 █  ██████╔╝██║░░██║╚█████╔╝██║░░██║██║░╚═╝░██║███████╗ █
 █  ╚═════╝░╚═╝░░╚═╝░╚════╝░╚═╝░░╚═╝╚═╝░░░░░╚═╝╚══════╝ █
 █                                                      █
 █  ░██████╗░█████╗░░█████╗░███╗░░██╗███╗░░██╗███████╗██████╗
 █  ██╔════╝██╔══██╗██╔══██╗████╗░██║████╗░██║██╔════╝██╔══██╗
 █  ╚█████╗░██║░░╚═╝███████║██╔██╗██║██╔██╗██║█████╗░░██████╔╝
 █  ░╚═══██╗██║░░██╗██╔══██║██║╚████║██║╚████║██╔══╝░░██╔══██╗
 █  ██████╔╝╚█████╔╝██║░░██║██║░╚███║██║░╚███║███████╗██║░░██║
 █  ╚═════╝░░╚════╝░╚═╝░░╚═╝╚═╝░░╚══╝╚═╝░░╚══╝╚══════╝╚═╝░░╚═╝
 █                                                      █
 █              >> Network Port Scanner <<              █
 ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
"#);

    println!("Enter The Ip Address: ");


    let mut target_ip = String::new();
    io::stdin().read_line(&mut target_ip);
    let target_ip = target_ip.trim();


    println!("Enter Range Port:(0 - n) ");

    let mut final_port = String::new();
    io::stdin().read_line(&mut final_port);


    let final_port = final_port.trim().parse::<i64>().unwrap();
    let mut open_ports = Vec::new();


    let mut port = 1;
    
    let bar = ProgressBar::new(final_port as u64);

    while port <= final_port {
       bar.inc(1); 
        let address: SocketAddr = format!("{}:{}", target_ip, port).parse().unwrap();
        
        let timeout = Duration::from_millis(60);
        let result = TcpStream::connect_timeout(&address, timeout);
        match result {
            Ok(_stream) => {
                open_ports.push(port);
            }
            Err(_e)=>{}
        }
        port += 1;

    }
    bar.finish();
        
    println!("\n=== Summary ===");
    println!("Found {} open ports: {:?}", open_ports.len(), open_ports);
}
