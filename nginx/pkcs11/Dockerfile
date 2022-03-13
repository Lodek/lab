FROM archlinux:latest

ARG CERT_DIR=/etc/certs
ARG SOFTHSM_PIN=1234
ARG PK_ID=1234
ARG KMS_URL=https://github.com/GoogleCloudPlatform/kms-integrations/releases/download/v1.0/libkmsp11-1.0-linux-amd64.tar.gz


# Install deps
RUN curl -s "https://archlinux.org/mirrorlist/?country=BR&protocol=http&protocol=https&ip_version=4" | sed -e 's/^#Server/Server/' -e '/^#/d' > /etc/pacman.d/mirrorlist
RUN pacman -Sy --noconfirm nginx softhsm libp11 vi && mkdir /etc/nginx/logs && touch /etc/nginx/logs/nginx.pid


# Configure OpenSSL pkcs11 engine and nginx
COPY openssl.cnf /etc/ssl/openssl.cnf
COPY nginx.conf /etc/nginx/nginx.conf


# Nginx Logs
RUN touch /var/log/nginx/access.log && ln -sf /dev/stdout /var/log/nginx/access.log && touch /var/log/nginx/error.log && ln -sf /dev/stdout /var/log/nginx/error.log


# Install and configure KMS
ENV KMS_PKCS11_CONFIG=/etc/kms.d/kms.yml
ENV GOOGLE_APPLICATION_CREDENTIALS=/etc/kms.d/credentials.json
RUN dir="/libkmsp11-1.0-linux-amd64" &&\
    tar="kms.tar.gz" && \
    curl -L "$KMS_URL" > $tar && \
    tar xvf $tar  && \
    cp $dir/libkmsp11.so /usr/lib/pkcs11/ && \
    chmod +x /usr/lib/pkcs11/libkmsp11.so && \
    rm -rf $dir $tar
RUN mkdir -p /etc/kms && touch /etc/kms/credentials.json
COPY kms.yml /etc/kms.d/kms.yml
COPY kms.module /usr/share/p11-kit/modules/kms.module
#COPY credentials.json /etc/kms.d/credentials.yml


# Generate certificate for file certificate
RUN mkdir $CERT_DIR
RUN openssl req -x509 -sha256 -nodes -days 365 -subj "/CN=file.example.com" \
                                               -newkey rsa:4096 \
                                               -keyout "$CERT_DIR/file.key" \
                                               -out "$CERT_DIR/file.crt"


# Generate certificate and add it to softhsm
RUN openssl req -x509 -sha256 -nodes -days 365 -subj "/CN=hsm.example.com" \
                                               -newkey rsa:4096 \
                                               -keyout "$CERT_DIR/hsm.key" \
                                               -out "$CERT_DIR/hsm.crt" && \
    openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in "$CERT_DIR/hsm.key" -out "$CERT_DIR/hsm.pkcs8.key" && \
    token_init_result=$(softhsm2-util --pin $SOFTHSM_PIN --so-pin $SOFTHSM_PIN --init-token --slot 0 --label "hsm") && \
    slot=$(echo $token_init_result | awk '{ print $11 }') && \
    echo "TOKEN SLOT: $slot" && \
    softhsm2-util --pin $SOFTHSM_PIN --so-pin $SOFTHSM_PIN --import "$CERT_DIR/hsm.key" --label "key" --id $PK_ID --slot $slot 


# Generate certificate from signing key in KMS
#RUN openssl req -new -x509 -subj '/CN=kms.example.com/' -sha256 -engine pkcs11 -keyform engine -key "pkcs11:object=ecc-hsm" > $CERT_DIR/kms.crt


CMD ["nginx"]
