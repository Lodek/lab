#!/usr/bin/sh
docker-compose run nginx openssl req -new -x509 -subj '/CN=kms.example.com/' -sha256 -engine pkcs11 -keyform engine -key "pkcs11:object=ecc-hsm" > kms.crt
