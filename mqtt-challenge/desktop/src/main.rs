use std::io::Error;
use tokio::net::TcpStream;
use mqtt_core::add;

use mqtt_core::writers::MqttWriter::MqttWriter;

use embedded_svc::io::asynch::{Read, Write};
use embedded_svc::io::{Io, ReadExactError};

struct KBTcpStream(tokio::net::TcpStream);

impl Io for KBTcpStream { type Error = (); }

impl Read for KBTcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.try_read(buf).map_err(|e| ())
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        self.0.try_read(buf).map(|_| ()).map_err(|e| ReadExactError::Other(()))
    }
}

impl Write for KBTcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.try_write(buf).map_err(|e| ())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.poll_flush().map(|r| ()).map_err(|e| ())
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.try_write(buf).map(|r| ()).map_err(|e| ())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world! {}", add(3,4));

    let mut recv_buffer = [0; 80];
    let mut write_buffer = [0; 80];
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let socket = KBTcpStream(socket);
    let mut mqtt_writer = MqttWriter::new("clientId-8rhWgBODCl", socket, &mut write_buffer, 80, &mut recv_buffer, 80);
    // match mqtt_writer.connect().await {
    //     Err(e) => continue,
    //     _ => {}
    // }

    Ok(())
}
