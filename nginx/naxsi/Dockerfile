FROM nginx:1.21-alpine as builder
ARG NAXSI_COMMIT

RUN apk add --update --no-cache \
                gcc \
                libc-dev \
                make \
                openssl-dev \
                pcre-dev \
                zlib-dev \
                linux-headers \
                libxslt-dev \
                gd-dev \
                geoip-dev \
                perl-dev \
                libedit-dev \
                mercurial \
                bash \
                alpine-sdk \
                findutils \
                git \
    && git clone https://github.com/nbs-system/naxsi.git \
    && if [ -n $NAXSI_COMMIT ]; then cd naxsi && git checkout $NAXSI_COMMIT && cd ..; fi \
    && wget https://nginx.org/download/nginx-${NGINX_VERSION}.tar.gz -O /nginx.tar.gz \
    && tar -xf /nginx.tar.gz \
    && cd nginx-${NGINX_VERSION} \
    && nginx_modules=$(nginx -V 2>&1 | sed -n 's/.*: \(--.*\)/\1/p') \
    && eval "./configure --add-dynamic-module=/naxsi/naxsi_src $nginx_modules" \
    && make modules \
    && cp objs/ngx_http_naxsi_module.so /etc/nginx/modules \
    && sed -i '1s|^|load_module /etc/nginx/modules/ngx_http_naxsi_module.so;\n|' /etc/nginx/nginx.conf \
    && nginx -t

FROM nginx:1.21-alpine
COPY --from=builder /nginx-${NGINX_VERSION}/objs/ngx_http_naxsi_module.so /etc/nginx/modules/
COPY --from=builder /naxsi/naxsi_config/naxsi_core.rules /etc/nginx/
RUN wget "https://gist.githubusercontent.com/Lodek/66a207c3e59076a297c0cd22141519dc/raw/fa119f402735e26e3038013e72e6bd6413638abc/nginx.conf" -O /etc/nginx/nginx.conf
