use crate::args::{TenantConfig, TenantConfigBuilder};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, DnValue, KeyIdMethod, KeyPair,
    SanType, PKCS_ECDSA_P384_SHA384,
};

pub struct Entity {
    pub cert: Certificate,
}

impl Entity {
    pub fn new(common_name: String) -> Self {
        let mut params = CertificateParams::new(vec![common_name.to_owned()]);
        params
            .distinguished_name
            .push(DnType::CommonName, common_name);
        Self {
            cert: Certificate::from_params(params).unwrap(),
        }
    }

    pub fn create_csr(&self) -> String {
        self.cert.serialize_request_pem().unwrap()
    }
}
