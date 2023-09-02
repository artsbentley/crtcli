mod ca_cert;

use ca_cert::Ca;
use std::fs;

fn main() {
    let ca = Ca::create();

    let cert = ca.cert.serialize_pem().unwrap();
    let private_key = ca.cert.serialize_private_key_pem();

    fs::create_dir_all("certs/").unwrap();

    fs::write("certs/rootca.pem", cert).unwrap();
    fs::write("certs/rootca.key", private_key).unwrap()
}

//     let mut dn = DistinguishedName::new();
//     dn.push(
//         DnType::CommonName,
//         DnValue::PrintableString("Test Root CA ECC".to_string()),
//     );
//
//     // SAN
//     let alternative_name = SanType::DnsName(String::from("test"));
//     let san: Vec<SanType> = vec![alternative_name];
//
//     // adjust params
//     let mut params = CertificateParams::new(vec!["test".to_string()]);
//     params.distinguished_name = dn;
//     params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
//     params.subject_alt_names = san;
//
//     // // gen seperate private key
//     // let key = KeyPair::generate(&PKCS_ECDSA_P256_SHA256).unwrap();
//     // let pem = key.serialize_pem();
//
//     // cert
//     let cert = Certificate::from_params(params).unwrap();
//
//     let cert_pem = cert.serialize_pem().unwrap();
//     let private_key = cert.serialize_private_key_pem();
//
//     println!("{private_key}");
//     println!("{cert_pem}");
//     // println!("{}", pem);
// }
