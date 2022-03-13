#!/usr/bin/sh
openssl genpkey -algorithm EC -out ec.key  -pkeyopt ec_paramgen_curve:P-384  -pkeyopt ec_param_enc:named_curve
openssl req -x509 -subj '/CN=test.com' -key ec.key
