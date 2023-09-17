use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CrtCliArgs {
    #[clap(subcommand)]
    pub cert_type: CertificateType,
}

#[derive(Debug, Subcommand)]
pub enum CertificateType {
    /// Create, sign and store a server certificate
    Server(ServerCommand),

    /// Create, sign and store a client certificate
    Client(ClientCommand),

    /// Create, sign and store a CA certificate
    Ca(CaCommand),
}

#[derive(Debug, Args)]
pub struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerSubCommand,
}

#[derive(Debug, Args)]
pub struct ClientCommand {
    #[clap(subcommand)]
    pub command: ClientSubCommand,
}

#[derive(Debug, Args)]
pub struct CaCommand {
    #[clap(subcommand)]
    pub command: CaSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ClientSubCommand {
    /// Create, sign and store a server certificate
    Create(CreateClientCert),
    /// Create, sign and store a client certificate
    Renew(RenewClientCert),
}

#[derive(Debug, Subcommand)]
pub enum ServerSubCommand {
    /// Create, sign and store a server certificate
    Sign(ServerCertConfig),
    /// Create, sign and store a client certificate
    Csr(CreateServerCsr),
}

#[derive(Debug, Subcommand)]
pub enum CaSubCommand {
    /// Create, sign and store a server certificate
    Create(CreateCa),
    /// Create, sign and store a client certificate
    Sign(SignWithCa),
}

#[derive(Debug, Args)]
pub struct CreateClientCert {
    /// for now any random string for testing purposes
    pub name: String,
}
#[derive(Debug, Args)]
pub struct RenewClientCert {
    /// for now any random string for testing purposes
    pub name: String,
}

#[derive(Debug, Args, Clone)]
pub struct ServerCertConfig {
    /// for now any random string for testing purposes
    #[clap(long, default_value = "test")]
    pub tenantname: String,
    #[clap(long, default_value = "poc")]
    pub environment: String,
    // #[clap(PossibleValue::new("dev", "test", "prod"))]
    #[clap(long, default_value = "test")]
    pub passphrase: String,
    #[clap(long, default_value = "broker")]
    pub broker_prefix: String,
    #[clap(long, default_value = "10")]
    pub broker_amount: u8,
    #[clap(long, default_value = "false")]
    pub inject_dsh: bool,
}

//     .name("tenantname".to_string())
//     .environment(Environment::POC)
//     .passphrase("test".to_string())
//     .broker_prefix("broker-1".to_string())
//     .broker_amount(BrokerAmount::Custom(3))
//     .inject_dsh(InjectDSH::False)
//
//
#[derive(Debug, Args, Clone)]
pub struct CreateServerCsr {
    /// for now any random string for testing purposes
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CreateCa {
    /// for now any random string for testing purposes
    pub name: String,
}
#[derive(Debug, Args)]
pub struct SignWithCa {
    /// for now any random string for testing purposes
    pub name: String,
}

// #[derive(Debug, Args)]
// pub struct ServerCommand {
//     /// Name of the tenant
//     #[clap(short, long)]
//     pub tenant_name: String,
//     /// Environment of the tenant
//     #[clap(short, long)]
//     pub environment: Environment,
//     /// Passphrase for the private key
//     #[clap(short, long)]
//     pub passphrase: String,
//     /// Amount of brokers to create
//     #[clap(short, long)]
//     pub broker_amount: BrokerAmount,
//     /// Prefix for the broker names
//     #[clap(short, long)]
//     pub broker_prefix: String,
//     /// Inject DSH into the certificate
//     #[clap(short, long)]
//     pub inject_dsh: InjectDSH,
// }
