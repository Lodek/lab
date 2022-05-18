# About
A CentOS 7 based Docker container with Nginx, OpenSSL 1.1.1, lip11, p11-kit and libkmsp11.

The container compiles OpenSSL 1.1.1 and compiles a minimal nginx against openssl 1.1.1.
libp11 and p11-kit are also compiled and installed under /usr/local.
libkmsp11 is fetched from Github Releases.

## Setup notes
The setup process combines a few different libs, environemtn variables and config files.

Since this is a functional docker environemnt, checkout the corresponding files in the repo to see a working example

### Env vars
Some environment variables are paramount for this integration to work.

- `OPENSSL_CONF` must point to an openssl conf which configure the pkcs11 engine
- `GOOGLE_APPLICATION_CREDENTIALS` must point to a valid GCP service account json credentials file. Note that the service account must have sufficient permissions
- `KMS_PKCS11_CONFIG` must point to a libkmsp11 yaml configuration file. See Libkmsp11 conf section
- `GRPC_ENABLE_FORK_SUPPORT` was recommended as part of libkms official integration guide. This seems like a somewhat obscure feature and I am unsure how it impacts the nginx integration. See https://github.com/grpc/grpc/issues/14056 and https://cloud.google.com/kms/docs/reference/pkcs11-nginx

`OPENSSL_CONF` must be set at a master process level.
In a dockerless environment the system.d nginx unit is a good option.

The following snippet is an example on how to setup the environment variables:
```
Environment="KMS_PKCS11_CONFIG=/etc/kms/kms.yml"
Environment="OPENSSL_CONF=/etc/ssl/openssl-pkcs11.cnf"
Environment="GOOGLE_APPLICATION_CREDENTIALS=/etc/gcp/credentials.json"
Environment="GRPC_ENABLE_FORK_SUPPORT=1"
```

Finally, some of these variables must be replicated to nginx's configuration file through the `env` directive.
That is because of nginx's behavior which removes env variables from workers environments
Therefore we need to replicate those values such that they will be accessible by the workers.
See: https://nginx.org/en/docs/ngx_core_module.html#env

### Configuration files

#### Libkmsp11 
Libkmsp11 requires 2 configuration files:
- kms.yml config file
- credentials.json credentials file

The credentials files is used by libkmsp11 to authenticate with GCP.
The credentials file contains the credentials for a GCP service account (https://cloud.google.com/iam/docs/service-accounts)

The kms yaml configuration file is specific for libkmsp11 (https://github.com/GoogleCloudPlatform/kms-integrations/blob/master/kmsp11/docs/user_guide.md).


Important notes:
- Ensure the kms configuration files are chowned to the `nginx` user.
- Ensure the kms.yml configuration file is chmoded to 744



### openssl
Openssl config file is used to declare and specify the location of the pkcs11 engine and which module it should use.

See:
- https://www.openssl.org/docs/man1.1.1/man5/config.html#Engine-Configuration-Module
- https://github.com/OpenSC/libp11#using-the-engine-from-the-command-line

### nginx
Nginx requires a few tweaks to work with the PCKS11 engine.
TLDR: add the `ssl_engine pkcs11` directive to the main context; add the correct `env ____` directive for the environment variables previously mentioned.

See:
- https://nginx.org/en/docs/ngx_core_module.html#env
- https://nginx.org/en/docs/ngx_core_module.html#ssl_engine


## PKCS11 logging
p11-kit is a proxy pkcs11 implementation which delegates calls to some underlying token.
It acts as a facilitator when multiple libs in the same process require pcks11 tokens.
For the purposes of this environment, p11-kit was added solemnly for logging capabilities and is not required.

More info at:
https://p11-glue.github.io/p11-glue/p11-kit/manual/

## Libkmsp11 conf
`libkmsp11` uses a `.yml` configuration file.
The config directives are docuemented in their [repository](https://github.com/GoogleCloudPlatform/kms-integrations/blob/master/kmsp11/docs/user_guide.md)

One caveat found with this integration is that libkmsp11 does not play nice when `refresh_interval_secs` is set.
It behaves correctl when Nginx is set to have no workers processes - ie `master_process off;`.
However, if the master process is on, then it causes issues.

The followings logs were retrieved with `master_process on;` and `refresh_interval_secs: 20`:

```
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:25 [debug] 13#13: epoll timer: -1
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: epoll: fd:13 ev:0001 d:0000000001806210
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: accept on 0.0.0.0:443, ready: 0
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: posix_memalign: 00000000017FFB80:512 @16
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 accept: 172.25.0.1:50802 fd:3
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 event timer add: 3: 60000:23251256
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 reusable connection: 1
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 epoll add event: fd:3 op:1 ev:80002001
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: timer delta: 5854
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: worker cycle
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: epoll timer: 60000
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: epoll: fd:3 ev:0001 d:00000000018064F8
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 http check ssl handshake
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 http recv(): 1
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 https ssl handshake: 0x16
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 tcp_nodelay
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 SSL server name: "foo.com"
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 SSL ALPN supported by client: h2
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 SSL ALPN supported by client: http/1.1
centos7-pkcs11-nginx-1  | 2022/05/18 11:09:31 [debug] 13#13: *1 SSL ALPN selected: http/1.1
centos7-pkcs11-nginx-1  | C_Initialize
centos7-pkcs11-nginx-1  |   IN: pInitArgs = NULL
centos7-pkcs11-nginx-1  | W20220518 11:09:55.971971    16 ssl_transport_security.cc:510] Corruption detected.
centos7-pkcs11-nginx-1  | W20220518 11:09:55.972086    16 ssl_transport_security.cc:486] error:1e000065:Cipher functions:OPENSSL_internal:BAD_DECRYPT
centos7-pkcs11-nginx-1  | W20220518 11:09:55.972110    16 ssl_transport_security.cc:486] error:1000008b:SSL routines:OPENSSL_internal:DECRYPTION_FAILED_OR_BAD_RECORD_MAC
centos7-pkcs11-nginx-1  | W20220518 11:09:55.972142    16 secure_endpoint.cc:208] Decryption error: TSI_DATA_CORRUPTED
```
Note that the `C_Initialize` was performed only when a request required access to a private key under the `pkcs11` Engine.


## Note on libkmsp11 + nginx reload
Upon receiving a `reload` command, it's Nginx's nature to kill the worker processes and start new ones.
This is an important observation when we consider the fact that each worker process will have to re-initialize their pkcs11 providers.

This has pros and cons.
As a pro, it means that even without `refresh_interval_secs` being set, the keyring will be updated once per reload.
In environments with frequent reloads, that means keys won't be stale.
On the downside, this will increase the reload duration, as libkmsp11 fetches all HSM objects from the KMS API upon start.
With a considerate (100+) ammount of keys in a keyring, we observed `C_Initialize` calls that can take up to 40 seconds.
