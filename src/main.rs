use std::net::UdpSocket;
use std::time::Duration;
use::std::process::exit;
use std::hash::{Hasher,Hash};
use std::collections::hash_map::DefaultHasher;

// #[derive(Hash)]
fn calculate_checksum<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()

}

// #[derive(Hash)]
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:12345")?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    
    let mut buf = [0; 1024];
    let mut retry_count = 0;

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                // println!("{:?}",&buf);
                let recevied_checksum = u64::from_be_bytes(buf[amt-12..amt-4].try_into().unwrap());
                // println!("{:?}",&buf[amt-12..amt-4]);
                // println!("the checksem is {:?}", recevied_checksum);
                let data = &buf[..amt-12];
                println!("{:?}",data);
                let string_data = std::str::from_utf8(&buf[..amt-12]).unwrap().to_string();
                println!("{:?}",string_data);
                let sequence_number = u32::from_be_bytes(buf[amt-4..amt].try_into().unwrap()); 
                println!("{:?}",sequence_number);

                //check the checksum
                if calculate_checksum(&string_data) != recevied_checksum {
                    println!("the checksum is {:?}", calculate_checksum(&data));
                    eprintln!("Checksum does not match, data might be corrupted");
                    // exit(1);
                } else {
                    println!("Checksum matches, data is intact");
                }
                // let data = &buf[..amt];
                // println!("Received {0:?} from {1:?}", data, src);
                match std::str::from_utf8(&data) {
                    Ok(s) => println!("sqquence_nember is {:?} and Received: {}",sequence_number,s),
                    Err(e) => eprintln!("Error converting data: {:?}", e),
                }

                // Send ACK
                socket.send_to(b"ACK", src)?;
                if string_data.trim() == "exit" {
                    println!("bye bye!");
                    exit(1);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                retry_count += 1;
                println!("retry counts {:?}",retry_count);
                if retry_count >= 5 {
                    eprintln!("faiiled to receive data after 5 attempts, exiting");
                    exit(1);
                }
                // Timeout occurred, continue listening
            }
            Err(e) => {
                eprintln!("Error receiving data: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}