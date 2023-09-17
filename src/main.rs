#![allow(unused_imports, dead_code)]

mod args;
mod ca_cert;
mod cli;
mod entity;
mod server_cert;

use args::{BrokerAmount, Environment, InjectDSH, TenantConfig, TenantConfigBuilder};
use ca_cert::Ca;
use entity::Entity;
use server_cert::ServerCertificate;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::fs;
use std::process::exit;

use clap::Parser;
use cli::*;

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

    let args: CrtCliArgs = CrtCliArgs::parse();

    match args.cert_type {
        CertificateType::Server(server_command) => {
            match server_command.command {
                ServerSubCommand::Sign(config) => {
                    // TODO: get the PossibleValues attribute to work, this solution is horrible
                    let environment = Environment::from_str(&config.environment).unwrap();
                    println!("{:?} {:?}", config.tenantname, config.broker_amount);

                    // TODO: use existing CA or give option for user to decide
                    let ca = Ca::new();

                    // TODO: parse the argument inputs to builder
                    let server_config = TenantConfigBuilder::new()
                        .name(config.tenantname)
                        // TODO: match statement to select environment enum from cli args
                        .environment(environment)
                        // NOTE: passphrase not being used at the moment, rcgen might have difficulties in leaving
                        // a fingerprint on the private key, see: https://stackoverflow.com/questions/72635424/how-to-create-a-fingerprint-in-rust-for-a-certficate-generated-with-the-rcgen-cr
                        .passphrase(config.passphrase)
                        .broker_prefix(config.broker_prefix)
                        // TODO: might enum might not be the solution for this
                        .broker_amount(BrokerAmount::Custom(config.broker_amount))
                        .inject_dsh(InjectDSH::False)
                        .build();

                    let server = ServerCertificate::new(server_config);

                    let server_key = server.cert.serialize_private_key_pem();
                    let server_csr = server.create_csr();
                    let server_cert = ca.sign_cert(&server.cert);

                    fs::write("certs/server.pem", &server_cert).unwrap();
                    fs::write("certs/servercsr.pem", &server_csr).unwrap();
                    fs::write("certs/server.key", &server_key).unwrap();

                    println!("{server_cert} {server_key}");
                    // TODO: save + validate certs
                }
                ServerSubCommand::Csr(_config) => {}
            }
        }
        CertificateType::Client(client_command) => match client_command.command {
            ClientSubCommand::Create(_config) => {}
            ClientSubCommand::Renew(_config) => {}
        },
        CertificateType::Ca(ca_command) => match ca_command.command {
            CaSubCommand::Create(_config) => {}
            CaSubCommand::Sign(_config) => {}
        },
    }

    // // CA
    // let ca = Ca::new();
    // let ca_cert = ca.cert.serialize_private_key_pem();
    // let ca_key = ca
    //     .cert
    //     .serialize_pem()
    //     .expect("umable to load cert into pem");

    // ENTITY
    // let entity = Entity::new();
    // let entity_key = entity.cert.serialize_private_key_pem();
    // let entity_csr = entity.create_csr();
    // let entity_cert = ca.sign_cert(&entity.cert);

    // // SERVER
    // let server_config = TenantConfigBuilder::new()
    //     .name("tenantname".to_string())
    //     .environment(Environment::POC)
    //     // NOTE: passphrase not being used at the moment, rcgen might have difficulties in leaving
    //     // a fingerprint on the private key, see: https://stackoverflow.com/questions/72635424/how-to-create-a-fingerprint-in-rust-for-a-certficate-generated-with-the-rcgen-cr
    //     .passphrase("test".to_string())
    //     .broker_prefix("broker-1".to_string())
    //     .broker_amount(BrokerAmount::Custom(3))
    //     .inject_dsh(InjectDSH::False)
    //     .build();
    //
    // let server = ServerCertificate::new(server_config);
    // let server_key = server.cert.serialize_private_key_pem();
    // let server_csr = server.create_csr();
    // let server_cert = ca.sign_cert(&server.cert);
    //
    // println!("{server_cert} {server_key}");
    //
    // fs::create_dir_all("certs/").unwrap();
    //
    // fs::write("certs/entity.pem", entity_cert).unwrap();
    // fs::write("certs/entitycsr.pem", entity_csr).unwrap();
    // fs::write("certs/entity.key", entity_key).unwrap();
    //
    // fs::write("certs/server.pem", server_cert).unwrap();
    // fs::write("certs/servercsr.pem", server_csr).unwrap();
    // fs::write("certs/server.key", server_key).unwrap();
}
