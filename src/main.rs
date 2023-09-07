mod args;
mod ca_cert;
mod entity;
mod server_cert;

use args::{BrokerAmount, Environment, InjectDSH, TenantBuilder, TenantConfig};
use ca_cert::Ca;
use entity::Entity;
use server_cert::ServerCertificate;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::fs;

const DB_URL: &str = "sqlite://sqlite.db";
#[tokio::main]
async fn main() {
    // SQL
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    // fn main() {

    // CA

    let ca = Ca::new();
    // let ca_cert = ca.cert.serialize_private_key_pem();
    // let ca_key = ca
    //     .cert
    //     .serialize_pem()
    //     .expect("umable to load cert into pem");

    // ENTITY
    let entity = Entity::new();
    let entity_key = entity.cert.serialize_private_key_pem();
    let entity_csr = entity.create_csr();
    let entity_cert = ca.sign_cert(&entity.cert);

    // TODO: try test out all of the builder patterns
    // SERVER
    let server_config = TenantBuilder::new()
        .name("tenantname".to_string())
        .environment(Environment::POC)
        .passphrase("test".to_string())
        .broker_prefix("helloooo".to_string())
        .broker_amount(BrokerAmount::Custom(3))
        .inject_dsh(InjectDSH::False)
        .build();

    let server = ServerCertificate::new(server_config);
    let server_key = server.cert.serialize_private_key_pem();
    let server_csr = server.create_csr();
    let server_cert = ca.sign_cert(&server.cert);

    println!("{server_cert} {server_key}");

    fs::create_dir_all("certs/").unwrap();

    fs::write("certs/entity.pem", entity_cert).unwrap();
    fs::write("certs/entitycsr.pem", entity_csr).unwrap();
    fs::write("certs/entity.key", entity_key).unwrap();

    fs::write("certs/server.pem", server_cert).unwrap();
    fs::write("certs/servercsr.pem", server_csr).unwrap();
    fs::write("certs/server.key", server_key).unwrap();
}
