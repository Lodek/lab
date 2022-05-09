FROM centos:centos7
ARG BUILD_DIR=/build
WORKDIR /build

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
RUN yum clean all &&\
    yum install --assumeyes curl &&\
    yum groupinstall --assumeyes "Development tools" &&\
    cd $BUILD_DIR &&\
    curl --location $OPENSSL_URL > $BUILD_DIR/openssl.tar.gz &&\
    tar xvf $BUILD_DIR/openssl.tar.gz &&\
    rm -f $BUILD_DIR/openssl.tar.gz && cd $BUILD_DIR/openssl* &&\
    ./config --prefix=$OPENSSL_PREFIX -fPIC &&\
    make && make install


# Build libp11
ARG LIBP11_RELEASE="https://github.com/OpenSC/libp11/releases/download/libp11-0.4.11/libp11-0.4.11.tar.gz"
RUN cd $BUILD_DIR &&\
    curl --location $LIBP11_RELEASE > $BUILD_DIR/libp11.tar.gz &&\
    tar xvf $BUILD_DIR/libp11.tar.gz &&\
    rm -f $BUILD_DIR/libp11.tar.gz &&\
    cd $BUILD_DIR/libp11* &&\
    ./configure OPENSSL_CFLAGS="-I${OPENSSL_PREFIX}/include" \
                OPENSSL_LIBS="-L${OPENSSL_PREFIX}/lib -lssl -lcrypto" \
                --with-enginesdir="${OPENSSL_PREFIX}/lib64/engines-1.1" \
    &&\
    make &&\
    make install


# Fetch and install libkmsp11 under /usr/local/lib
ARG libkmsp11_url="https://github.com/GoogleCloudPlatform/kms-integrations/releases/download/v1.1/libkmsp11-1.1-linux-amd64.tar.gz"
RUN cd $BUILD_DIR &&\
    curl --location $libkmsp11_url > $BUILD_DIR/libkms.tar.gz &&\
    tar xvf $BUILD_DIR/libkms.tar.gz &&\
    rm -f $BUILD_DIR/libkms.tar.gz &&\
    mkdir -p /etc/kms /etc/gcp &&\
    mv $BUILD_DIR/libkmsp11*/libkmsp11.so /usr/local/lib/libkmsp11.so &&\
    rm -rf $BUILD_DIR/libkms*

    
# Minimum Nginx build from source.
# Uses the same version as being used by azion, sans the custom patches and modules.
# See: https://github.com/aziontech/azion-nginx/blob/develop/specs/nginx.spec#L40
#
# Note that this nginx is dynamically linking against openssl and azion's nginx
# statically builds nginx.
# This build was done because centos 7 nginx runs against openssl 1.0.x,
# which is incompatible with pkcs11 engine and libkmsp11
ARG nginx_url="https://nginx.org/download/nginx-1.16.1.tar.gz"
RUN cd $BUILD_DIR &&\
    yum install --assumeyes pcre2 pcre pcre-devel pcre2-devel zlib-devel &&\
    curl --location $nginx_url > $BUILD_DIR/nginx.tar.gz &&\
    tar xvf $BUILD_DIR/nginx.tar.gz &&\
    rm -f  $BUILD_DIR/nginx.tar.gz && cd $BUILD_DIR/nginx* &&\
    ./configure --with-http_ssl_module \
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
    ln -sf /dev/stderr /var/log/nginx/error.log &&\
    ln -sf /dev/stdin /var/log/nginx/access.log

ENV OPENSSL_CONF="/etc/openssl/openssl-pkcs11.cnf"
ENV LD_LIBRARY_PATH="/usr/local/lib64"
COPY ./conf/nginx/nginx.conf /etc/nginx/nginx.conf
CMD ["/usr/local/nginx/sbin/nginx", "-g", "daemon off;", "-c", "/etc/nginx/nginx.conf"]
