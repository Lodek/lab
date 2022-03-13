# About
Environment using Nginx, libp11 ssl engine, p11-kit and libkmsp11

# Setup

## KMS
- Enable kms product
- Create keyring
- Create `EC-P256-SHA256` signing key (key name matches `object` pkcs11 attribute)

## Service Account
- Create service account
- Give service account permission to list and perform signing operations over a keyring


## Credetials
- Generate key for service account
- Save `json` file with service account credentials to the root of the repository with the name `credentials.json`

## Certificate and nginx configuration
- Generate certificate for signing key stored in KMS. `gen-kms-cert.sh`. (substitute `object` parameter to match the key in keyring)
- Update pkcs11 uri in nginx.conf to match key in keyring
