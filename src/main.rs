mod ca_cert;
mod entity;

use ca_cert::Ca;
use entity::Entity;
use entity::ServerCertificate;
use std::fs;

fn main() {
    // CA
    let ca = Ca::new();
    let ca_cert = ca.cert.serialize_pem().unwrap();
    let ca_private_key = ca.cert.serialize_private_key_pem();

    // ENTITY
    let entity = Entity::new();
    let entity_key = entity.cert.serialize_private_key_pem();
    let entity_csr = entity.create_csr();
    let entity_cert = ca.sign_cert(&entity.cert);

    // SERVER
    let server = ServerCertificate::new();
    let server_key = server.cert.serialize_private_key_pem();
    let server_csr = server.cert.serialize_pem().unwrap();
    let server_cert = ca.sign_cert(&server.cert);

    println!("{server_key}{server_csr} {server_cert}");

    fs::create_dir_all("certs/").unwrap();
    fs::write("certs/rootca.pem", ca_cert).unwrap();
    fs::write("certs/rootca.key", ca_private_key).unwrap();

    fs::write("certs/entity.pem", entity_cert).unwrap();
    fs::write("certs/entitycsr.pem", entity_csr).unwrap();
    fs::write("certs/entity.key", entity_key).unwrap();

    fs::write("certs/server.pem", server_cert).unwrap();
    fs::write("certs/servercrs.pem", server_csr).unwrap();
    fs::write("certs/server.key", server_key).unwrap();
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
