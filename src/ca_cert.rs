use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, DnValue, IsCa, KeyIdMethod, KeyPair,
    KeyUsagePurpose, SanType, PKCS_ECDSA_P256_SHA256,
};

use std::{env, fs};

pub struct Ca {
    pub cert: Certificate,
}

impl Ca {
    pub fn create() -> Ca {
        let key_pair = KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();

        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, "rootca");

        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.distinguished_name = dn;
        params.key_pair = Some(key_pair);
        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(365 * 20);
        params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];

        let cert = Certificate::from_params(params).unwrap();
        // let cert_pem = cert.serialize_pem().unwrap();

        Ca { cert }
    }
}

// fs::create_dir_all("certs/").unwrap();
// fs::write("certs/rootca.pem", cert_pem.as_bytes()).unwrap();
// fs::write(
//     "certs/rootca.key",
//     cert.serialize_private_key_pem().as_bytes(),
// )
// .unwrap();
//
