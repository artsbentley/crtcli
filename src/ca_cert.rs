use rcgen::{
    Certificate, CertificateParams, DistinguishedName, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose,
};
use std::env;
use std::fs;
use std::path::Path;

// use std::{env, fs};

pub struct Ca {
    pub cert: Certificate,
}

impl Ca {
    pub fn new() -> Ca {
        if Self::exists() {
            println!("CA certificate exists");
            return Self::from_file();
        }
        println!("CA certificate does not exist");
        Self::create_ca()
    }

    // exists: return enum with string

    pub fn exists() -> bool {
        let ca_cert_path = "certs/rootca.pem";
        Path::new(ca_cert_path).exists()
    }

    pub fn from_file() -> Ca {
        let ca_cert_path = fs::read_to_string("certs/rootca.pem").unwrap();
        let ca_key_path = fs::read_to_string("certs/rootca.key").unwrap();

        let ca_key = KeyPair::from_pem(&ca_key_path).unwrap();

        let ca_cert_params = CertificateParams::from_ca_cert_pem(&ca_cert_path, ca_key).unwrap();
        let cert = Certificate::from_params(ca_cert_params).unwrap();

        Ca { cert }
    }

    pub fn create_ca() -> Ca {
        // CA configuration
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

        let mut dn = DistinguishedName::new();

        let which = env::var("USER").unwrap();
        dn.push(rcgen::DnType::CommonName, which);
        params.distinguished_name = dn;

        let key_pair = KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        params.key_pair = Some(key_pair);
        params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
        params.use_authority_key_identifier_extension = true;
        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(365 * 20);
        params.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::CrlSign,
            KeyUsagePurpose::DigitalSignature,
            KeyUsagePurpose::KeyEncipherment,
        ];
        params.extended_key_usages = vec![
            ExtendedKeyUsagePurpose::ServerAuth,
            ExtendedKeyUsagePurpose::ClientAuth,
        ];

        let cert = Certificate::from_params(params).unwrap();
        // let cert_pem = cert.serialize_pem().unwrap();

        // TODO: wrap creation of files into function
        let ca_cert = cert.serialize_pem().unwrap();
        let ca_private_key = cert.serialize_private_key_pem();

        fs::write("certs/rootca.pem", ca_cert).unwrap();
        fs::write("certs/rootca.key", ca_private_key).unwrap();
        Ca { cert }
    }

    pub fn sign_cert(&self, cert: &Certificate) -> String {
        cert.serialize_pem_with_signer(&self.cert).unwrap()
    }
}
