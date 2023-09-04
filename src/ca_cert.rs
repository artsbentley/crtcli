use rcgen::{
    Certificate, CertificateParams, DistinguishedName, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose,
};

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
        Ca { cert }
    }

    pub fn sign_cert(&self, cert: &Certificate) -> String {
        cert.serialize_pem_with_signer(&self.cert).unwrap()
    }
}
