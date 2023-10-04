// *
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

// TODO: add duration of validity for the cert
#[derive(Clone)]
pub struct TenantConfig {
    pub name: String,
    pub environment: Environment,
    pub passphrase: String,
    pub broker_prefix: String,
    pub broker_amount: BrokerAmount,
    pub inject_dsh: InjectDSH,
}

impl TenantConfig {
    // name for local storage of certs
    pub fn format_save_name(&self, file_postfix: String) -> String {
        format!(
            "{}_{}_{}{}",
            self.name,
            self.environment.to_str(),
            "server",
            file_postfix
        )
    }

    // name that will be used to upload to DSH secrets
    pub fn format_dsh_secret_name(&self, file_postfix: String) -> String {
        format!("{}_kafkaproxy-{}", self.broker_prefix, file_postfix)
    }

    // name for local storage of certs
    pub fn format_directory_location(&self) -> String {
        format!("{}/{}/server/", self.environment.to_str(), self.name,)
    }
}

#[derive(Copy, Clone)]
pub enum Environment {
    POC,
    PROD,
    PUBLIC,
    PRODLZ,
    NPLZ,
}

impl Environment {
    pub fn url(&self) -> String {
        match self {
            // TODO: make sure capitalization works
            Environment::POC => "poc.kpn-dsh.com".to_string(),
            Environment::PROD => "TODO".to_string(),
            Environment::PRODLZ => "dsh-prod.dsh.prod.aws.kpn.com".to_string(),
            Environment::PUBLIC => "prod.cp.kpn-dsh.com".to_string(),
            Environment::NPLZ => "TODO".to_string(),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Environment::POC => "poc".to_string(),
            Environment::PROD => "prod".to_string(),
            Environment::PRODLZ => "prodlz".to_string(),
            Environment::PUBLIC => "public".to_string(),
            Environment::NPLZ => "nplz".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "poc" => Ok(Environment::POC),
            "prod" => Ok(Environment::PROD),
            "prodlz" => Ok(Environment::PRODLZ),
            "public" => Ok(Environment::PUBLIC),
            "nplz" => Ok(Environment::NPLZ),
            _ => Err("Invalid environment option. Choose from ..."),
        }
    }

    pub fn bearer_endpoint(&self) -> String {
        match self {
            Environment::POC => "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token".to_string(),
            Environment::PROD => "todo".to_string(),
            Environment::PRODLZ => "https://auth.lz.lz-cp.dsh.np.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token ".to_string(),
            Environment::PUBLIC => "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token".to_string(),
            Environment::NPLZ => "todo".to_string(),
        }
    }

    pub fn api_client_id(&self) -> String {
        match self {
            Environment::POC => "poc-dsh".to_string(),
            Environment::PROD => "prod-dsh".to_string(),
            Environment::PRODLZ => "prod-lz-dsh".to_string(),
            Environment::PUBLIC => "prod-dsh".to_string(),
            Environment::NPLZ => "nplz-dsh".to_string(),
        }
    }
}

// TODO: reconsider if this should be an enum
#[derive(Clone)]
pub enum BrokerAmount {
    CSRDefault,
    SelfSignedDefault,
    Custom(u8),
}

impl BrokerAmount {
    pub fn get(&self) -> u8 {
        match self {
            BrokerAmount::CSRDefault => 10,
            BrokerAmount::SelfSignedDefault => 12,
            BrokerAmount::Custom(amount) => *amount,
        }
    }
}

#[derive(Clone)]
pub enum InjectDSH {
    True(String),
    False,
}

pub struct TenantConfigBuilder {
    pub name: Option<String>,
    pub environment: Option<Environment>,
    pub passphrase: Option<String>,
    pub broker_prefix: Option<String>,
    pub broker_amount: Option<BrokerAmount>,
    pub inject_dsh: Option<InjectDSH>,
}

impl TenantConfigBuilder {
    pub fn new() -> Self {
        TenantConfigBuilder {
            name: None,
            environment: None,
            passphrase: None,
            broker_prefix: None,
            broker_amount: None,
            inject_dsh: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn passphrase(mut self, passphrase: String) -> Self {
        self.passphrase = Some(passphrase);
        self
    }

    pub fn broker_prefix(mut self, broker_prefix: String) -> Self {
        self.broker_prefix = Some(broker_prefix);
        self
    }

    pub fn broker_amount(mut self, broker_amount: BrokerAmount) -> Self {
        self.broker_amount = Some(broker_amount);
        self
    }

    pub fn inject_dsh(mut self, inject_dsh: InjectDSH) -> Self {
        self.inject_dsh = Some(inject_dsh);
        self
    }

    pub fn build(self) -> TenantConfig {
        TenantConfig {
            name: self.name.unwrap_or_default(),
            environment: self.environment.unwrap_or(Environment::POC),
            passphrase: self.passphrase.unwrap_or_default(),
            broker_prefix: self.broker_prefix.unwrap_or_default(),
            broker_amount: self.broker_amount.unwrap_or(BrokerAmount::CSRDefault),
            inject_dsh: self.inject_dsh.unwrap_or(InjectDSH::False),
        }
    }
}
