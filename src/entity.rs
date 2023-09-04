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
    pub fn new() -> Self {
        let mut params = CertificateParams::default();

        params.alg = &PKCS_ECDSA_P384_SHA384;
        params.key_pair = Some(KeyPair::generate(&PKCS_ECDSA_P384_SHA384).unwrap());
        params.key_identifier_method = KeyIdMethod::Sha256;

        // create fn get_

        params.distinguished_name = Self::get_distinguished_name("localhost".to_string());
        params.subject_alt_names = Self::get_brokers(10, "test".to_string());

        let cert = Certificate::from_params(params).unwrap();
        ServerCertificate { cert }
    }

    pub fn get_distinguished_name(common_name: String) -> DistinguishedName {
        let mut dn = DistinguishedName::new();

        dn.push(DnType::CommonName, DnValue::PrintableString(common_name));
        dn.push(
            DnType::OrganizationName,
            DnValue::PrintableString("Koninklijke KPN N.V.".to_string()),
        );
        dn.push(
            DnType::CountryName,
            DnValue::PrintableString("NL".to_string()),
        );
        dn.push(
            DnType::LocalityName,
            DnValue::PrintableString("Rotterdam".to_string()),
        );
        dn.push(
            DnType::StateOrProvinceName,
            DnValue::PrintableString("Zuid-Holland".to_string()),
        );

        dn
    }

    pub fn get_brokers(n: u8, broker_name: String) -> Vec<SanType> {
        (1..=n)
            .map(|i| {
                let dns_name = format!("broker-{}.kafka.{}", broker_name, i);
                SanType::DnsName(dns_name)
            })
            .collect()
        // let brokers = vec![SanType::DnsName("localhost".to_string())];
        // brokers
    }
}
