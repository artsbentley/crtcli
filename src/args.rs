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
// */
struct Tenant {
    name: String,
    environment: Environment,
    passphrase: String,
    broker_prefix: String,
    broker_amount: u8,
    inject_dsh: InjectDSH,
}

enum Environment {
    POC(String),
    PROD(String),
    PROD_LZ(String),
    NP_LZ(String),
}

enum InjectDSH {
    True(String),
    False,
}
