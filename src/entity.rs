use crate::args::{TenantBuilder, TenantConfig};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, DnValue, KeyIdMethod, KeyPair,
    SanType, PKCS_ECDSA_P384_SHA384,
};

pub struct Entity {
    pub cert: Certificate,
}

impl Entity {
    pub fn new() -> Self {
        let mut params = CertificateParams::new(vec!["entity.xavamedia.nl".to_owned()]);
        params
            .distinguished_name
            .push(DnType::CommonName, "entity.xavamedia.nl");
        Self {
            cert: Certificate::from_params(params).unwrap(),
        }
    }

    pub fn create_csr(&self) -> String {
        self.cert.serialize_request_pem().unwrap()
    }
}

// pub struct ServerCertificate {
//     pub cert: Certificate,
// }
//
// /*
// * - Tenant name?
// * - Which tenant environment?
// *   url
// *   common name
// *   api_client_id
// *
// *
// * - How many brokers do you want?
// * - Broker prefix
// *
// * - passphrase
// * - do you want to use self signed certs?
// *
// * - prompt for API keyp
// */
//
// impl ServerCertificate {
//     pub fn new(config: TenantConfig) -> Self {
//         let mut params = CertificateParams::default();
//
//         params.not_before = time::OffsetDateTime::now_utc();
//         params.not_after = time::OffsetDateTime::now_utc() + time::Duration::days(365 * 20);
//
//         params.alg = &PKCS_ECDSA_P384_SHA384;
//         params.key_pair = Some(KeyPair::generate(&PKCS_ECDSA_P384_SHA384).expect("NOT WORKING"));
//         params.key_identifier_method = KeyIdMethod::Sha256;
//
//         // DN
//         let url = config.environment.url();
//         let dn_postfix = format!("{}.{}", config.name, &url);
//         // println!("{dn_postfix}");
//
//         params.distinguished_name = Self::extend_distinguished_name(dn_postfix);
//         // Self::get_distinguished_name("broker.kafka.asml-01.poc.kpn-dsh.com".to_string());
//
//         // SAN
//         // TODO: remove hardcoded borker values
//         let san_postfix = format!("{}.{}", config.name, &url);
//         let broker_amount = config.broker_amount.get();
//         let broker_prefix = config.broker_prefix;
//
//         params.subject_alt_names = Self::get_brokers(broker_amount, broker_prefix, san_postfix);
//
//         let cert = Certificate::from_params(params).unwrap();
//         ServerCertificate { cert }
//     }
//
//     pub fn extend_distinguished_name(common_name: String) -> DistinguishedName {
//         let mut dn = DistinguishedName::new();
//
//         dn.push(DnType::CommonName, DnValue::PrintableString(common_name));
//         dn.push(DnType::OrganizationName, "Koninklijke KPN N.V.");
//         dn.push(DnType::CountryName, "NL");
//         dn.push(DnType::LocalityName, "Rotterdam");
//         dn.push(DnType::StateOrProvinceName, "Zuid-Holland");
//         dn
//     }
//
//     pub fn get_brokers(n: u8, broker_prefix: String, tenant_name: String) -> Vec<SanType> {
//         (0..=n)
//             .map(|i| {
//                 let dns_name = format!("{}-{}.kafka.{}", broker_prefix, i, tenant_name);
//                 SanType::DnsName(dns_name)
//             })
//             .collect()
//         // let brokers = vec![SanType::DnsName("localhost".to_string())];
//         // brokers
//     }
//
//     pub fn create_csr(&self) -> String {
//         self.cert.serialize_request_pem().unwrap()
//     }
// }
