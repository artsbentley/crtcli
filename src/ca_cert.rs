use rcgen::{
    Certificate, CertificateParams, CertificateSigningRequest, DistinguishedName, IsCa, KeyPair,
    KeyUsagePurpose,
};
use x509_parser::certification_request::X509CertificationRequest;
use x509_parser::prelude::FromDer;

// use std::{env, fs};

pub struct Ca {
    pub cert: Certificate,
}

impl Ca {
    pub fn new() -> Ca {
        // CA configuration
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, "rootca");
        params.distinguished_name = dn;

        let key_pair = KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        params.key_pair = Some(key_pair);

        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(365 * 20);
        params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];

        let cert = Certificate::from_params(params).unwrap();
        // let cert_pem = cert.serialize_pem().unwrap();

        Ca { cert }
    }
    pub fn sign_cert(&self, csr_pem: &str) -> String {
        let csr_der = x509_parser::pem::parse_x509_pem(csr_pem.as_bytes())
            .unwrap()
            .1;
        let csr = X509CertificationRequest::from_der(&csr_der.contents)
            .unwrap()
            .1;
        // csr.verify_signature().unwrap();
        let csr = CertificateSigningRequest::from_der(&csr_der.contents).unwrap();
        let csr = CertificateSigningRequest::serialize_pem_with_signer(&self, &self.cert);
        csr.serialize_pem_with_signer(&self.certificate).unwrap()
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
