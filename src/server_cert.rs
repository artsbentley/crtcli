use crate::args::{TenantConfig, TenantConfigBuilder};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, DnValue, ExtendedKeyUsagePurpose,
    KeyIdMethod, KeyPair, KeyUsagePurpose, SanType, SerialNumber, PKCS_ECDSA_P384_SHA384,
    PKCS_RSA_SHA256,
};

use picky::key::PrivateKey;
use picky::x509::csr::Csr;
use picky::x509::name::{DirectoryName, NameAttr};
use picky::{hash::HashAlgorithm, signature::SignatureAlgorithm};

use tracing::info;

pub struct ServerCertificate {
    pub cert: Certificate,
}

impl ServerCertificate {
    pub fn new(config: &TenantConfig) -> Self {
        let mut params = CertificateParams::default();

        // TODO: read lifetime from config, adjust config to contain it aswell
        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(30);

        // NOTE: with these old configurations below no exponent is generated, which is needed for
        // the signing of KPN CA, also RCGEN does not scupport a private key algorithm below 2048,
        // hence we are using openssl crate:
        // params.key_pair = Some(KeyPair::generate(&PKCS_ECDSA_P384_SHA384).expect("NOT WORKING"));
        // params.alg = &PKCS_ECDSA_P384_SHA384;

        params.alg = &PKCS_RSA_SHA256;
        params.key_identifier_method = KeyIdMethod::Sha256;

        // rcgen doesnt have 2048 generator, hence we use openssl crate
        let pkey: openssl::pkey::PKey<_> = openssl::rsa::Rsa::generate(2048)
            .unwrap()
            .try_into()
            .unwrap();
        let key_pair_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let key_pair = rcgen::KeyPair::from_pem(&key_pair_pem).unwrap();

        // // picky key creation
        // info!("Generating keypair");
        // let pkey = Self::generate_keypair().unwrap();
        // let key_pair_pem = pkey.to_pem_str().unwrap();
        // let key_pair = rcgen::KeyPair::from_pem(&key_pair_pem).unwrap();
        //
        params.key_pair = Some(key_pair);

        let broker_prefix = config.broker_prefix.clone();
        let url = config.environment.url();

        // let serial_num: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // let serial = SerialNumber::from(serial_num);
        // params.serial_number = Some(serial);
        params.serial_number = None;

        // WARNING: key usage and key extended usage is not supported for CSR in the RCGEN crate,
        // therefor a CSR is impossiple currently
        // https://github.com/rustls/rcgen/pull/107/commits/9572fd985c39bfc70adeab1ec0272888e9bc93dd

        // // Key usage
        // params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        // params.key_usages.push(KeyUsagePurpose::KeyEncipherment);
        // params.key_usages.push(KeyUsagePurpose::KeyAgreement);
        //
        // // Extended key usage
        // params
        //     .extended_key_usages
        //     .push(ExtendedKeyUsagePurpose::ServerAuth);

        // DN
        let common_name = format!("{}.kafka.{}.{}", &broker_prefix, config.name, &url);
        params.distinguished_name = Self::extend_distinguished_name(common_name);

        // SAN
        let broker_amount = config.broker_amount.get();
        let san_postfix = format!("{}.{}", config.name, &url);
        params.subject_alt_names = Self::get_brokers(broker_amount, broker_prefix, san_postfix);

        // TODO: read out passphrase from config and create encrypted key with openssl commands:
        // https://stackoverflow.com/questions/72635424/how-to-create-a-fingerprint-in-rust-for-a-certficate-generated-with-the-rcgen-cr

        let cert = Certificate::from_params(params).unwrap();
        ServerCertificate { cert }
    }

    pub fn extend_distinguished_name(common_name: String) -> DistinguishedName {
        let mut dn = DistinguishedName::new();

        dn.push(DnType::CommonName, DnValue::PrintableString(common_name));
        dn.push(DnType::OrganizationName, "Koninklijke KPN N.V.");
        dn.push(DnType::CountryName, "NL");
        dn.push(DnType::LocalityName, "Rotterdam");
        dn.push(DnType::StateOrProvinceName, "Zuid-Holland");
        dn
    }

    pub fn get_brokers(n: u8, broker_prefix: String, tenant_name: String) -> Vec<SanType> {
        (0..=n - 1)
            .map(|i| {
                let dns_name = format!("{}-{}.kafka.{}", broker_prefix, i, tenant_name);
                SanType::DnsName(dns_name)
            })
            .collect()
        // let brokers = vec![SanType::DnsName("localhost".to_string())];
        // brokers
    }

    pub fn create_csr(&self) -> String {
        self.cert.serialize_request_pem().unwrap()
    }

    /// Generate a keypair with Picky
    pub fn generate_keypair() -> Result<PrivateKey, Box<dyn std::error::Error>> {
        let bits: usize = 4096;
        let private_key = PrivateKey::generate_rsa(bits)?;
        Ok(private_key)
    }
}
