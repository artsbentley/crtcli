use crate::args::{TenantConfig, TenantConfigBuilder};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, DnValue, KeyIdMethod, KeyPair,
    SanType, PKCS_ECDSA_P384_SHA384,
};

pub struct ServerCertificate {
    pub cert: Certificate,
}

/*
* - Tenant name?
* - Which tenant environment?
*   url
*   common name
*   api_client_id
*
*
* - How many brokers do you want?
* - Broker prefix
*
* - passphrase
* - do you want to use self signed certs?
*
* - prompt for API keyp
*/

impl ServerCertificate {
    pub fn new(config: TenantConfig) -> Self {
        let mut params = CertificateParams::default();

        // TODO: read lifetime from config, adjust config to contain it aswell
        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(30);

        // NOTE: more research needed about the chosen encryption algorithms, current choice is at
        // random
        params.alg = &PKCS_ECDSA_P384_SHA384;
        params.key_pair = Some(KeyPair::generate(&PKCS_ECDSA_P384_SHA384).expect("NOT WORKING"));
        params.key_identifier_method = KeyIdMethod::Sha256;

        let broker_prefix = config.broker_prefix;
        let url = config.environment.url();

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
        (0..=n)
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
}
