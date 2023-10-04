#![allow(unused_imports, dead_code)]

mod api;
mod args;
mod ca_cert;
mod cli;
mod entity;
mod server_cert;

use api::dsh_api::DshApi;

use args::{BrokerAmount, Environment, InjectDSH, TenantConfig, TenantConfigBuilder};
use ca_cert::Ca;
use entity::Entity;
use server_cert::ServerCertificate;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::fs;
use std::process::exit;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use clap::Parser;
use cli::*;

const DB_URL: &str = "sqlite://sqlite.db";
#[tokio::main]
async fn main() {
    // TRACING
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // SQL
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    // CLI
    let args: CrtCliArgs = CrtCliArgs::parse();

    // NOTE: there is no way the current command should take as long as it does, not being run
    // async?
    match args.cert_type {
        // SERVER
        CertificateType::Server(server_command) => {
            match server_command.command {
                // SIGN SERVER
                ServerSubCommand::Sign(config) => {
                    // TODO: get the PossibleValues attribute to work, this solution is suboptimal
                    let environment = Environment::from_str(&config.environment).unwrap();

                    let inject_dsh = match config.inject_dsh.as_str() {
                        "false" => InjectDSH::False,
                        _ => InjectDSH::True(config.inject_dsh),
                    };

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
                        .broker_amount(BrokerAmount::Custom(config.broker_amount))
                        .inject_dsh(inject_dsh)
                        .build();

                    // e4LADzqpkxvh0GdG8uIy8IKrAaf5A3xm

                    let server = ServerCertificate::new(&server_config);

                    // NOTE: consideration; wrapping the end result certs in an own struct, with
                    // addional methods to handle the certs, such as saving and validating
                    let server_key = server.cert.serialize_private_key_pem();
                    let server_csr = server.create_csr();
                    let server_cert = ca.sign_cert(&server.cert);

                    // TODO: match the enum generated above if a DSHAPI struct and functions
                    // should be created and called

                    // REST calls
                    match server_config.inject_dsh {
                        InjectDSH::True(_) => {
                            let key_name = &server_config.format_dsh_secret_name("key".into());
                            let cert_name = &server_config.format_dsh_secret_name("cert".into());
                            let ca_name = &server_config.format_dsh_secret_name("ca".into());

                            let mut api = DshApi::new(&server_config);
                            // let bearer = api.retrieve_token().await.unwrap();
                            api.initialize_bearer_token().await.unwrap();

                            api.send_secret(ca_name, &ca.get_pem()).await.unwrap();
                            api.send_secret(cert_name, &server_cert).await.unwrap();
                            api.send_secret(key_name, &server_key).await.unwrap();
                            api.create_dsh_cert(&server_config.broker_prefix, cert_name, key_name)
                                .await
                                .unwrap();
                        }
                        InjectDSH::False => {}
                    }

                    // FILE CREATION
                    let directory = &server_config.format_directory_location();
                    fs::create_dir_all(&directory).unwrap();

                    let key_name = &server_config.format_save_name(".key.pem".into());
                    let csr_name = &server_config.format_save_name(".csr.pem".into());
                    let cert_name = &server_config.format_save_name(".crt.pem".into());

                    fs::write(directory.clone() + cert_name, &server_cert).unwrap();
                    fs::write(directory.clone() + csr_name, &server_csr).unwrap();
                    fs::write(directory.clone() + key_name, &server_key).unwrap();

                    println!("{server_cert}");
                    // TODO: save + validate certs
                }

                // CREATE SERVER CSR
                /*
                 X509v3 Basic Constraints:
                    CA:FALSE
                X509v3 Key Usage:
                    Digital Signature, Key Encipherment, Key Agreement
                X509v3 Extended Key Usage: critical
                    TLS Web Server Authentication
                */
                ServerSubCommand::Csr(config) => {
                    let environment = Environment::from_str(&config.environment).unwrap();
                    let server_config = TenantConfigBuilder::new()
                        .name(config.tenantname)
                        // TODO: match statement to select environment enum from cli args
                        .environment(environment)
                        // NOTE: passphrase not being used at the moment, rcgen might have difficulties in leaving
                        // a fingerprint on the private key, see: https://stackoverflow.com/questions/72635424/how-to-create-a-fingerprint-in-rust-for-a-certficate-generated-with-the-rcgen-cr
                        .passphrase(config.passphrase)
                        .broker_prefix(config.broker_prefix)
                        .broker_amount(BrokerAmount::CSRDefault)
                        .build();

                    let server = ServerCertificate::new(&server_config);

                    // NOTE: consideration; wrapping the end result certs in an own struct, with
                    // addional methods to handle the certs, such as saving and validating
                    let server_key = server.cert.serialize_private_key_pem();
                    let server_csr = server.create_csr();

                    // TODO: create a text with the instructions for singing of the cert through
                    // KPN  as a CA -> also add instructions to view the cert wih openssl commands
                    // TODO: create naming convention fo the certs close to the config struct

                    // NOTE: cannot create multiple overlapping files because of naming
                    // conevntion; create a date format aswell

                    // FILE CREATION
                    let directory = &server_config.format_directory_location();
                    println!("{directory}");
                    fs::create_dir_all(&directory).unwrap();

                    let key_name = &server_config.format_save_name(".key.pem".into());
                    let csr_name = &server_config.format_save_name(".csr.pem".into());

                    fs::write(directory.clone() + csr_name, &server_csr).unwrap();
                    fs::write(directory.clone() + key_name, &server_key).unwrap();

                    println!("{server_csr}");

                    // TODO: save + validate certs
                }
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
// fs::write("certs/entitycsr.pem", entityY_csr).unwrap();
// fs::write("certs/entity.key", entity_key).unwrap();
//
// fs::write("certs/server.pem", server_cert).unwrap();
// fs::write("certs/servercsr.pem", server_csr).unwrap();
// fs::write("certs/server.key", server_key).unwrap();
