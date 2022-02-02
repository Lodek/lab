FROM alpine:latest

ENV PKCS11_MOD=/usr/lib/softhsm/libsofthsm2.so

WORKDIR /app

RUN apk update && apk add cargo softhsm gnutls-utils openssl-dev openssl
RUN softhsm2-util --slot 0 --label ya --so-pin 1234 --pin 1234 --init-token &&  p11tool --login --generate-privkey=rsa --bit 2048 --label ok --set-pin=1234 pkcs11:token=ya

COPY . /app
RUN cargo build
CMD ["cargo", "run"]
