FROM golang:1.16-bullseye

ARG IGNITE_URL="https://github.com/ignite-hq/cli/releases/download/v0.17.3/starport_0.17.3_linux_amd64.tar.gz"

WORKDIR /app

RUN mkdir /ignite && cd /ignite &&\
    curl --location $IGNITE_URL > ignite.tar.gz &&\
    tar -xvf ignite.tar.gz &&\
    install --owner=root --group=root starport /usr/local/bin/ignite &&\
    cd / &&\
    rm -rf /ignite &&\
    apt-get update && apt-get --yes install jq

COPY . /app

EXPOSE 26657 1317 4500

CMD ["ignite", "chain", "serve"]
