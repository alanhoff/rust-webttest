use base64::engine::general_purpose::STANDARD as Base64Engine;
use base64::Engine;
use ring::digest::digest;
use ring::digest::SHA256;

pub trait CertificateInfo {
    fn der_chain(&self) -> Vec<Vec<u8>>;
    fn der_key(&self) -> Vec<u8>;
    fn fingerprint(&self) -> String;
}

pub struct SelfSignedCert {
    cert: rcgen::Certificate,
}

impl SelfSignedCert {
    pub fn new() -> Self {
        SelfSignedCert {
            cert: rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap(),
        }
    }
}

impl CertificateInfo for SelfSignedCert {
    fn der_chain(&self) -> Vec<Vec<u8>> {
        let chain = self.cert.serialize_der().unwrap();
        vec![chain]
    }

    fn der_key(&self) -> Vec<u8> {
        self.cert.serialize_private_key_der()
    }

    fn fingerprint(&self) -> String {
        let keypair = self.cert.get_key_pair();
        let digest = digest(&SHA256, &keypair.public_key_der());

        Base64Engine.encode(digest)
    }
}
