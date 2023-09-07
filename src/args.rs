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

pub struct TenantConfig {
    pub name: String,
    pub environment: Environment,
    pub passphrase: String,
    pub broker_prefix: String,
    pub broker_amount: BrokerAmount,
    pub inject_dsh: InjectDSH,
}

// impl TenantConfig {
//     fn get_common_name(&self) -> String {}
//
//     //example: broker.kafka.asml-01.poc.kpn-dsh.com
// }

pub enum Environment {
    POC,
    PROD,
    PRODLZ,
    NPLZ,
}

impl Environment {
    pub fn url(&self) -> String {
        match self {
            Environment::POC => "poc.kpn-dsh.com".to_string(),
            Environment::PROD => "prod".to_string(),
            Environment::PRODLZ => "prodlz".to_string(),
            Environment::NPLZ => "nplz".to_string(),
        }
    }
}

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
