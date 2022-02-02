use openssl::hash::MessageDigest;
use openssl::sign::Verifier;
use openssl::x509::X509;
use pkcs11;
use pkcs11::types::{
    CKA_CLASS, CKF_SERIAL_SESSION, CKM_RSA_PKCS, CKM_SHA1_RSA_PKCS, CKO_PRIVATE_KEY, CKU_SO,
    CKU_USER, CK_ATTRIBUTE, CK_MECHANISM, CK_OBJECT_HANDLE, CK_VOID, CK_VOID_PTR,
};
use pkcs11::Ctx;
use serde::Serialize;
use serde_json;
use std;
use std::env;
use std::io;
use std::io::Read;
use std::process;
use std::ptr::null;

type Result<T> = std::result::Result<T, pkcs11::errors::Error>;

const pin: &str = "1234";
const payload: &str = "input data ayaya";

fn main() {
    let module = env::var("PKCS11_MOD").unwrap();
    let mut ctx = Ctx::new_and_initialize(module.as_str()).unwrap();
    let slots_ids = ctx.get_slot_list(true).unwrap();

    let token_data = ctx.get_token_info(slots_ids[0]).unwrap();
    println!("{:?}", token_data);

    let session = ctx
        .open_session(slots_ids[0], CKF_SERIAL_SESSION, None, None)
        .unwrap();

    ctx.login(session, CKU_USER, Some(pin)).unwrap();

    let mut pkey_filter = CK_ATTRIBUTE::new(CKA_CLASS);
    pkey_filter.set_ck_ulong(&CKO_PRIVATE_KEY);
    let template = [pkey_filter];

    ctx.find_objects_init(session, &template).unwrap();
    let objs = ctx.find_objects(session, 10).unwrap();
    ctx.find_objects_final(session).unwrap();

    let pkey = objs[0];

    let mechanism = CK_MECHANISM {
        mechanism: CKM_SHA1_RSA_PKCS,
        pParameter: 0 as *mut CK_VOID,
        ulParameterLen: 0,
    };

    ctx.sign_init(session, &mechanism, pkey).unwrap();
    let signature = ctx.sign(session, payload.as_bytes()).unwrap();
    //why is this breaking? sign shouldn't finalize the operation :thinking:
    //ctx.sign_final(session).unwrap();
    for byte in signature {
        print!("{:X} ", byte);
    }
    println!("");

    ctx.close_session(session).unwrap();
    ctx.finalize().unwrap();
}

/*
Signature verification with openssl
fn main() {
    let mut buf: [u8; 10000] = [0; 10000];
    let mut stdin = io::stdin();
    stdin.read(&mut buf);

    let x509 = X509::from_pem(&buf).unwrap();
    let pkey = x509.public_key().unwrap();

    let _signed_data: [u8; 1000] = [0; 1000];
    let ref signed_data = _signed_data;

    let verifier = Verifier::new(MessageDigest::sha1(), pkey.as_ref()).unwrap();
    let result = verifier.verify(signed_data).unwrap();
}
*/

// WIP pkcs11 abstraction

/*
struct Token {
    session_handle: Option<u32>,
    ctx: Ctx,
}

impl struct Token {

    pub fn new(cryptoki_module: &str) -> Result<Self> {
        let ctx = Ctx::new_and_initialize("/usr/lib/pkcs11/libsofthsm2.so")?;
        Self {
            ctx,
            session_handle: None
        }
    }

    pub fn open_session(&mut self, slot_label: &str, user_pin: &str) -> Result<()> {
        let slots_ids = ctx.get_slot_list(true).unwrap();
        // opens session with first slot found
        let session = ctx.open_session(slots_ids[0], CKF_SERIAL_SESSION, None, null())?;
        self.session_handle = session
    }

    pub fn find_private_key_by_label<'a, 'b>(&'a mut self, label: &'b str) -> Result<PrivKey<'a>> {
        let pkey_filter = CK_ATTIRBUTE::new(CKA_CLASS);
        pkey_filter.set_ck_ulong(&CKO_PRIVATE_KEY);
        let template = [pkey_filter];

        self.ctx.find_objects_init(session, &template).unwrap();
        let objs = self.ctx.find_objects(session, 10).unwrap();
        ctx.find_objects_final(session).unwrap();
        let pkey = obs[0];

        PrivKey::new(&self.ctx, pkey)
        }
    }

}

struct Mechanism {

}

// this is problematic because ctx calls are mutable
// ie i'd have 2 mutable refs to ctx, no bueno
//
// Do I need a cell?

struct PrivKey<'a> {
    handle: CK_OBJECT_HANDLE,
    ctx: &'a Ctx
}

impl<'a> PrivKey<'a> {

    pub fn new(ctx: &'a Ctx, handle: CK_OBJECT_HANDLE) -> Self {
        Self { ctx, handle }
    }

    pub fn sign_data() -> Result<Vec<u8>> {
        Ok(Vec::new());
    }
}
*/
