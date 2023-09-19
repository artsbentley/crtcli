## Limitations:

- if a CA exists, it cannot be overwritten currently

## wishes:

- i want to be able to create a custom CA per tenant/ use-case so that the self signed certs dont have access to other tenants

- as a user i want to be able to supply configuration with flags, but also as an option with a (yaml) config file

- the server certificates that are generated, need to be persisted with the CA chain of the signer, so that this can always be queried and recovered easily


- as a user, i want to be aple to use the cli tool to easily validate or view the certificate/ csr generated
