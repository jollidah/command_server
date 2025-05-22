use crate::errors::ServiceError;
use openssl::{
    pkey::{Private, Public},
    rsa::{Padding, Rsa},
};

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key: Rsa<Public>,
}
impl PublicKey {
    pub fn from_pem(pem: &[u8]) -> Result<Self, ServiceError> {
        let key = Rsa::public_key_from_pem(pem)?;
        Ok(PublicKey { key })
    }
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub key: Rsa<Private>,
}

impl PrivateKey {
    pub fn from_pem(pem: &[u8]) -> Result<Self, ServiceError> {
        let key = Rsa::private_key_from_pem(pem)?;
        Ok(PrivateKey { key })
    }

    pub fn decode_data(&self, token: &[u8]) -> Result<String, ServiceError> {
        let mut buf: Vec<u8> = vec![0; self.key.size() as usize];
        let bytes = self.key.private_decrypt(token, &mut buf, Padding::PKCS1)?;
        let pt = String::from_utf8(buf[0..bytes].to_vec()).map_err(|_| ServiceError::ParseError)?;
        Ok(pt)
    }
}

pub struct VultrKeyPair;

impl VultrKeyPair {
    pub fn generate_key_pair() -> Result<(PublicKey, PrivateKey), ServiceError> {
        // TODO! raise issue to rsa crate (Using deprecated rand methods)
        let bits = 512;
        let private_key: Rsa<Private> = Rsa::generate(bits)?;

        let public_key_pem: Vec<u8> = private_key.public_key_to_pem()?;
        let public_key: Rsa<Public> = Rsa::public_key_from_pem(&public_key_pem)?;

        let public_key = PublicKey { key: public_key };
        let private_key = PrivateKey { key: private_key };
        Ok((public_key, private_key))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub async fn encode_data_with_public_key(public_key: PublicKey, data: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; public_key.key.size() as usize];
        let token_len = public_key
            .key
            .public_encrypt(data, &mut buf, Padding::PKCS1)
            .unwrap();
        buf[0..token_len].to_vec()
    }

    #[tokio::test]
    async fn test_decode_data() {
        let (public_key, private_key) = VultrKeyPair::generate_key_pair().unwrap();

        let test_string = "test String";
        let encoded_token = encode_data_with_public_key(public_key, test_string.as_bytes()).await;

        let decoded_token = private_key.decode_data(&encoded_token).unwrap();
        assert_eq!(decoded_token, test_string);
    }
    #[ignore]
    #[tokio::test]
    async fn test_encode_data_with_public_key() {
        let public_key = PublicKey::from_pem("test_public_key".as_bytes()).unwrap();
        let api_key = "test_api_key";
        let target = encode_data_with_public_key(public_key, api_key.as_bytes()).await;
        println!("{:?}", target);
    }
}
