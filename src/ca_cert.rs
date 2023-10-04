use rcgen::{
    Certificate, CertificateParams, DistinguishedName, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose,
};
use std::env;
use std::fs;
use std::path::Path;

use tracing::{info, warn};
// use std::{env, fs};

pub struct Ca {
    pub cert: Certificate,
}

impl Ca {
    // TODO: CA path shouldnt be hardcoded
    pub fn new() -> Ca {
        if Self::exists() {
            info!("CA certificate exists");
            return Self::from_file();
        }
        warn!("CA certificate does not exist");
        info!("Creating CA certificate");
        Self::create_ca()
    }

    pub fn get_path() -> String {
        let ca_cert_path = "certs/rootca.pem";
        ca_cert_path.to_string()
    }

    // NOTE:  better solution: exists; return enum with string of cert
    pub fn exists() -> bool {
        let ca_cert_path = "certs/rootca.pem";
        Path::new(ca_cert_path).exists()
    }

    // TODO: standardize the location of the certs/ input as variable
    pub fn from_file() -> Ca {
        let ca_cert_path = fs::read_to_string("certs/rootca.pem").unwrap();
        let ca_key_path = fs::read_to_string("certs/rootca.key").unwrap();

        let ca_key = KeyPair::from_pem(&ca_key_path).unwrap();

        let ca_cert_params = CertificateParams::from_ca_cert_pem(&ca_cert_path, ca_key).unwrap();
        let cert = Certificate::from_params(ca_cert_params).unwrap();

        Ca { cert }
    }

    pub fn get_pem(&self) -> String {
        self.cert.serialize_pem().unwrap()
    }

    pub fn create_ca() -> Ca {
        // CA configuration
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

        let mut dn = DistinguishedName::new();

        let which = env::var("USER").unwrap();
        dn.push(rcgen::DnType::CommonName, which);
        params.distinguished_name = dn;

        // let key_pair = KeyPair::generate(&rcgen::PKCS_RSA_SHA256).unwrap();
        // params.key_pair = Some(key_pair);

        let pkey: openssl::pkey::PKey<_> = openssl::rsa::Rsa::generate(2048)
            .unwrap()
            .try_into()
            .unwrap();
        let key_pair_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let key_pair = rcgen::KeyPair::from_pem(&key_pair_pem).unwrap();
        params.key_pair = Some(key_pair);

        params.alg = &rcgen::PKCS_RSA_SHA256;
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

        // TODO:: wrap creation fo files into function:
        //
        // let cert_pem = cert.serialize_pem().unwrap();
        let ca_cert = cert.serialize_pem().unwrap();
        let ca_private_key = cert.serialize_private_key_pem();

        fs::create_dir_all("certs/").unwrap();
        fs::write("certs/rootca.pem", ca_cert).unwrap();
        fs::write("certs/rootca.key", ca_private_key).unwrap();

        Ca { cert }
    }

    pub fn sign_cert(&self, cert: &Certificate) -> String {
        cert.serialize_pem_with_signer(&self.cert).unwrap()
    }
}
