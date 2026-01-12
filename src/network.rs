use std::net::TcpStream;

pub struct NetworkConnectionInfo {
    scheme: String,
    host: String,
    port: u16,
    path: String,
}

pub struct TCPClient {
    stream: TcpStream,
    info: NetworkConnectionInfo,
}

impl TCPClient {
    pub fn new(info: NetworkConnectionInfo) -> Result<TCPClient, String> {
        let stream = TcpStream::connect((info.host, info.port));
        match stream {
            Ok(stream) => Ok(TCPClient { stream, info }),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn send(&self, data: &str) -> Result<(), String> {
        self.stream.write_all(data.as_bytes());
        Ok(())
    }

    pub fn recv(&self, buffer_size: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0; buffer_size];
        let bytes_read = self.stream.read(&mut buffer).map_err(|e| e.to_string())?;

        if bytes_read == 0 {
            return Err("Connection closed".to_string());
        }

        Ok(buffer[..bytes_read].to_vec())
    }

    pub fn close(&self) -> Result<(), String> {
        self.stream.shutdown(std::net::Shutdown::Both);
        Ok(())
    }
}
