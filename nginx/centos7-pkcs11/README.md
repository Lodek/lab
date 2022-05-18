# About
A CentOS 7 based Docker container with Nginx, OpenSSL 1.1.1, lip11, p11-kit and libkmsp11.

The container compiles OpenSSL 1.1.1 and compiles a minimal nginx against openssl 1.1.1.
libp11 and p11-kit are also compiled and installed under /usr/local.
libkmsp11 is fetched from Github Releases.


## PKCS11 logging
p11-kit is a proxy pkcs11 implementation which delegates calls to some underlying token.
It acts as a facilitator when multiple libs in the same process require pcks11 tokens.
For the purposes of this environment, p11-kit was added solemnly for logging capabilities and is not required.

More info at:
https://p11-glue.github.io/p11-glue/p11-kit/manual/

## Libkmsp11 conf
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
