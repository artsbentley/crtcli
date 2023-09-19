#![allow(unused_imports, dead_code)]

mod api;
mod args;
mod ca_cert;
mod cli;
mod entity;
mod server_cert;

use api::dsh_api::{process_tls, DshApi};

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

    let args: CrtCliArgs = CrtCliArgs::parse();

    match args.cert_type {
        // SERVER
        CertificateType::Server(server_command) => {
            match server_command.command {
                // SIGN SERVER
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
                        .inject_dsh(InjectDSH::True(
                            "e4LADzqpkxvh0GdG8uIy8IKrAaf5A3xm".to_string(),
                        ))
                        .build();

                    let server = ServerCertificate::new(&server_config);

                    let server_key = server.cert.serialize_private_key_pem();
                    let server_csr = server.create_csr();
                    let server_cert = ca.sign_cert(&server.cert);

                    // let api = DshApi::new(
                    //     server_config.environment,
                    //     server_config.name,
                    //     "e4LADzqpkxvh0GdG8uIy8IKrAaf5A3xm".to_string(),
                    // );

                    let mut api = DshApi::new(&server_config);
                    let bearer = api.retrieve_token().await.unwrap();
                    api.initialize_bearer_token().await.unwrap();
                    api.send_secret("test", "test").await.unwrap();
                    println!("{bearer}");

                    fs::write("certs/server.pem", &server_cert).unwrap();
                    fs::write("certs/servercsr.pem", &server_csr).unwrap();
                    fs::write("certs/server.key", &server_key).unwrap();

                    let proccessed_tls = process_tls(&server_cert);
                    println!("{server_cert}");
                    println!("{proccessed_tls}");
                    fs::write("certs/api_servercsr.pem", &proccessed_tls).unwrap();
                    // println!("{server_cert} {server_key}");
                    // TODO: save + validate certs
                }

                // CREATE SERVER CSR
                ServerSubCommand::Csr(_config) => {}
            }
        }
        // CLIENT
        CertificateType::Client(client_command) => match client_command.command {
            // SIGN CLIENT
            ClientSubCommand::Sign(config) => {
                let ca = Ca::new();

                let entity = Entity::new(config.common_name);
                let entity_key = entity.cert.serialize_private_key_pem();
                let entity_csr = entity.create_csr();
                let entity_cert = ca.sign_cert(&entity.cert);

                println!("{entity_cert} {entity_key}");

                // println!("{:?}", token);

                fs::write("certs/entity.pem", entity_cert).unwrap();
                fs::write("certs/entitycsr.pem", entity_csr).unwrap();
                fs::write("certs/entity.key", entity_key).unwrap();
            }

            // RENEW CLIENT?
            ClientSubCommand::Renew(_config) => {}
        },

        // CA
        CertificateType::Ca(ca_command) => match ca_command.command {
            // CREATE CA
            CaSubCommand::Create(_config) => {}

            // SIGN ANYTHING WITH CA
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
