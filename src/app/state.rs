use crate::hash::hasher;
#[derive(Debug)]
pub struct AppState {
    pub input: String,
    pub result: String,
    pub algorithm: Algorithm,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            result: "".to_string(),
            algorithm: Algorithm::default(),
        }
    }
}

impl AppState {
    // pub fn new(input: String, algorithm: String) -> Self {
    //     Self {
    //         input,
    //         result: String::new(),
    //         algorithm: Algorithm::from(algorithm.as_str()),
    //     }
    // }
}
pub fn hash(input: &str, algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::Md5 => hasher::hash_md5(input),
        _ => todo!(),
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Algorithm {
    #[default]
    Md5,
    Md6,
    Sha1,
    Sha224,
    Sha256,
    Sha512,
    Tiger,
    Whirpool,
}

impl From<&str> for Algorithm {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "MD5" => Self::Md5,
            "MD6" => Self::Md6,
            "SHA1" | "SHA-1" => Self::Sha1,
            "SHA224" | "SHA-224" => Self::Sha224,
            "SHA256" | "SHA-256" => Self::Sha256,
            "SHA512" | "SHA-512" => Self::Sha512,
            "TIGER" => Self::Tiger,
            "WHIRPOOL" => Self::Whirpool,
            _ => panic!("Could not create algorithm from {value}"),
        }
    }
}
