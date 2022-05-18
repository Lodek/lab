FROM centos:centos7

# Initial setup
ARG BUILD_DIR=/build
WORKDIR /build
RUN yum clean all &&\
    yum --assumeyes install curl vim &&\
    yum groupinstall --assumeyes "Development tools" &&\
    groupadd --gid 101 nginx &&\
    useradd --shell /sbin/nologin --gid 101 --uid 101 nginx


# Build openssl
# 
# A note about openssl1.1.1 installation directory:
# I thought a lot about where to install, the 3 main options seemed to be:
# - /usr
# - /usr/local
# - /opt
# /opt is an alternative, however openssl does follow linux conventions for directory hierarchy.
# because of that I discarded /opt.
# Another option is `/usr`, and since we are creating an rpm package for this lib,
# it seemed like `/usr` would work.
# In theory it's possible because we can have libssl.so.1.1.1 as the filename and explicitly
# link against that version.
# That would solve the problem for the `.so` object, but this does not address the problem
# with `/usr/include` files, as there's only a single version.
# As such, I chose `/usr/local` as the installation prefix as its unix-y and does not cause conflits
# More info:
# - https://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html
# - https://www.pathname.com/fhs/pub/fhs-2.3.html#THEUSRHIERARCHY
ARG OPENSSL_URL="https://www.openssl.org/source/openssl-1.1.1o.tar.gz"
ARG OPENSSL_PREFIX="/usr/local"
ARG OPENSSL_DIR="/root"
RUN cd $BUILD_DIR &&\
    curl --location $OPENSSL_URL > $BUILD_DIR/openssl.tar.gz &&\
    tar xvf $BUILD_DIR/openssl.tar.gz &&\
    rm -f $BUILD_DIR/openssl.tar.gz && cd $BUILD_DIR/openssl* &&\
    ./config --prefix=$OPENSSL_PREFIX -fPIC &&\
    make && make install


# Minimum Nginx build from source linking against the compiled openssl 1.1.1
# Uses the same version as being used by azion, sans the custom patches and modules.
# See: https://github.com/aziontech/azion-nginx/blob/develop/specs/nginx.spec#L40
ARG nginx_url="https://nginx.org/download/nginx-1.16.1.tar.gz"
RUN cd $BUILD_DIR &&\
    yum install --assumeyes pcre2 pcre pcre-devel pcre2-devel zlib-devel vim &&\
    curl --location $nginx_url > $BUILD_DIR/nginx.tar.gz &&\
    tar xvf $BUILD_DIR/nginx.tar.gz &&\
    rm -f  $BUILD_DIR/nginx.tar.gz && cd $BUILD_DIR/nginx* &&\
    ./configure --conf-path=/etc/nginx/nginx.conf \
                --with-http_ssl_module \
                --with-http_v2_module \
                --with-http_realip_module \
                --with-http_addition_module \
                --with-http_gzip_static_module \
                --with-http_random_index_module \
                --with-http_secure_link_module \
                --with-http_degradation_module \
                --with-http_stub_status_module \
                --with-http_slice_module \
                --with-http_auth_request_module \
                --with-pcre \
                --with-threads \
                --with-debug &&\
    make && make install &&\
    mkdir -p /var/log/nginx &&\
    chown nginx:nginx /var/log/nginx &&\
    ln -sf /dev/stderr /var/log/nginx/error.log &&\
    ln -sf /dev/stdin /var/log/nginx/access.log &&\
    ln -sf /usr/local/nginx/sbin/nginx /sbin/nginx


# Build libp11
# Libp11 contains the OpenSSL pkcs11 engine which is used by nginx to perform
# crypto operations using HSM objects
ARG LIBP11_RELEASE="https://github.com/OpenSC/libp11/releases/download/libp11-0.4.11/libp11-0.4.11.tar.gz"
RUN cd $BUILD_DIR &&\
    curl --location $LIBP11_RELEASE > $BUILD_DIR/libp11.tar.gz &&\
    tar xvf $BUILD_DIR/libp11.tar.gz &&\
    rm -f $BUILD_DIR/libp11.tar.gz &&\
    cd $BUILD_DIR/libp11* &&\
    ./configure OPENSSL_CFLAGS="-I${OPENSSL_PREFIX}/include" \
                OPENSSL_LIBS="-L${OPENSSL_PREFIX}/lib64 -lssl -lcrypto" \
                --with-enginesdir="${OPENSSL_PREFIX}/lib64/engines-1.1" \
    &&\
    make &&\
    make install


# Fetch and install libkmsp11 under /usr/local/lib
# libkmsp11 is the pkcs11 cryptoki provider for Googles Cloud HSM
ARG libkmsp11_url="https://github.com/GoogleCloudPlatform/kms-integrations/releases/download/v1.1/libkmsp11-1.1-linux-amd64.tar.gz"
RUN cd $BUILD_DIR &&\
    curl --location $libkmsp11_url > $BUILD_DIR/libkms.tar.gz &&\
    tar xvf $BUILD_DIR/libkms.tar.gz &&\
    rm -f $BUILD_DIR/libkms.tar.gz &&\
    mkdir -p /etc/kms /etc/gcp &&\
    install --owner=nginx --group=nginx --mode=755 $BUILD_DIR/libkmsp11*/libkmsp11.so /usr/local/lib/libkmsp11.so &&\
    rm -rf $BUILD_DIR/libkms* &&\
    mkdir -p /var/log/kms &&\
    chown nginx:nginx /var/log/kms &&\
    mkdir -p /etc/gcp &&\
    mkdir -p /etc/kms


# build p11-kit
# p11 kit is an util which allows for better coordination between in process libs
# which call a pkcs11 module.
# We use this because it conveniently is able to log pkcs11 calls
ARG p11_kit_url="https://github.com/p11-glue/p11-kit/releases/download/0.24.1/p11-kit-0.24.1.tar.xz"
RUN yum --assumeyes install libtasn1-devel libffi-devel &&\
    cd $BUILD_DIR &&\
    curl --location $p11_kit_url > $BUILD_DIR/p11-kit.tar.gz &&\
    tar xvf $BUILD_DIR/p11-kit.tar.gz &&\
    rm -f $BUILD_DIR/p11-kit.tar.gz &&\
    cd $BUILD_DIR/p11-kit* &&\
    ./configure --prefix="/usr/local" --with-system-config="/etc/pkcs11" &&\
    make &&\
    make install


# Configure
#ENV KMS_PKCS11_CONFIG=/etc/kms/kms.yml
#ENV GOOGLE_APPLICATION_CREDENTIALS=/etc/gcp/credentials.json
#ENV GRPC_ENABLE_FORK_SUPPORT=1
#ENV OPENSSL_CONF="/etc/openssl/openssl-pkcs11.cnf"

COPY --chown=nginx:nginx ./conf/p11-kit /etc/pkcs11
COPY --chown=nginx:nginx ./conf/kms/kms.yml /etc/kms/kms.yml
COPY --chown=nginx:nginx ./secrets/credentials.json /etc/gcp/credentials.json
COPY --chown=nginx:nginx ./conf/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --chown=nginx:nginx ./conf/openssl/openssl-pkcs11.cnf /etc/openssl/openssl-pkcs11.cnf

RUN echo /usr/local/lib >> /etc/ld.so.conf &&\
    echo /usr/local/lib64 >> /etc/ld.so.conf &&\
    ldconfig  &&\
    chown nginx:nginx /etc/kms/kms.yml &&\
    chown nginx:nginx /etc/gcp/credentials.json

CMD KMS_PKCS11_CONFIG=/etc/kms/kms.yml GOOGLE_APPLICATION_CREDENTIALS=/etc/gcp/credentials.json GRPC_ENABLE_FORK_SUPPORT=1 OPENSSL_CONF="/etc/openssl/openssl-pkcs11.cnf" nginx -g 'daemon off;'
