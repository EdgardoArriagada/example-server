use std::io::Read;
use std::net::TcpListener;

use std::str::FromStr;

pub struct Server {
    addr: String,
}

struct KeyValue {
    key: String,
    value: String,
}

impl FromStr for KeyValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_val: Vec<&str> = s.split(' ').collect();
        
        Ok(KeyValue {
            key: key_val[0].to_string(),
            value: key_val[1].to_string(),
        })
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let data = &String::from_utf8_lossy(&buffer);
                            let tuple  = KeyValue::from_str(data).unwrap();
                            
                            println!("key: {}, value: {}", tuple.key, tuple.value);
                        },
                        Err(e) => println!("Failed to establish a connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
