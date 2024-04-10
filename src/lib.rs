wai_bindgen_rust::export!("universal-crypto.wai");

use secret_toolkit_crypto::sha_256;

pub struct UniversalCrypto;

impl universal_crypto::UniversalCrypto for UniversalCrypto {
    fn hash(input: String) -> String {
        base64::encode(sha_256(input.as_bytes()))
    }
}
