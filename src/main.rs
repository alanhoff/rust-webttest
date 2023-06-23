mod cert;

use cert::CertificateInfo;
use cert::SelfSignedCert;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use wtransport::tls::Certificate;
use wtransport::Endpoint;
use wtransport::ServerConfig;

async fn handle(connection: wtransport::Connection) {
    loop {
        let mut buffer = [0; 65536];

        tokio::select! {
            stream = connection.accept_bi() => {
                let mut stream = stream.unwrap();
                println!("Accepted BI stream");

                let bytes_read = stream.1.read(&mut buffer).await.unwrap().unwrap();
                let str_data = std::str::from_utf8(&buffer[..bytes_read]).unwrap();

                println!("Received (bi) '{str_data}' from client");

                stream.0.write_all(b"ACK").await.unwrap();
            }
            stream = connection.accept_uni() => {
                let mut stream = stream.unwrap();
                println!("Accepted UNI stream");

                let bytes_read = stream.read(&mut buffer).await.unwrap().unwrap();
                let str_data = std::str::from_utf8(&buffer[..bytes_read]).unwrap();

                println!("Received (uni) '{str_data}' from client");

                let mut stream = connection.open_uni().await.unwrap();
                stream.write_all(b"ACK").await.unwrap();
            }
            dgram = connection.receive_datagram() => {
                let dgram = dgram.unwrap();
                let str_data = std::str::from_utf8(&dgram).unwrap();

                println!("Received (dgram) '{str_data}' from client");

                connection.send_datagram(b"ACK").unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let self_signed = SelfSignedCert::new();
    let certificate = Certificate::new(self_signed.der_chain(), self_signed.der_key());

    println!("Certificate generated");
    println!("Fingerprint: {}", self_signed.fingerprint());

    let config = ServerConfig::builder()
        .with_bind_address(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433))
        .with_certificate(certificate);

    let server = Endpoint::server(config).unwrap();

    while let Some(connection) = server.accept().await {
        println!("Got connection");

        match connection.await {
            Ok(connection) => {
                tokio::spawn(async move {
                    handle(connection).await;
                });
            }
            Err(err) => {
                eprintln!("Error: {:?}", err)
            }
        };
    }
}
