wai_bindgen_rust::export!("universal-crypto.wai");

use secret_toolkit_crypto::sha_256;
use universal_crypto::Crab;

pub struct UniversalCrypto;

pub struct Beach {
    crabs: Vec<Crab>,
}

impl universal_crypto::Beach for Beach {
    fn is_ferris_here(&self) -> bool {
        self.crabs.iter().any(|crab| crab.name == "Ferris")
    }
}

impl universal_crypto::UniversalCrypto for UniversalCrypto {
    fn hash(input: String) -> String {
        base64::encode(sha_256(input.as_bytes()))
    }

    /// This function will return a `Crab` struct with the name "Ferris" and age 2.
    fn get_ferris() -> Crab {
        Crab {
            age: 2,
            name: String::from("Ferris"),
        }
    }
}
