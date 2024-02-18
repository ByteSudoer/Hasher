use sha1::Digest;
pub fn hash_md5(input: &str) -> String {
    let data = input.as_bytes();
    let digest = md5::compute(data);
    let result = format!("{digest:x}");
    tracing::info!("MD5 Hashed String {input} to {result}");
    result
}

pub fn hash_md6(input: &str) -> String {
    let data = input.as_bytes();
    let mut result = [0; 32];
    let _ = md6::hash(256, data, &mut result);
    let result: String = Iterator::map(result.iter(), |byte| format!("{:02X}", byte)).collect();

    tracing::info!("MD6 Hashed String {input} to {result}");
    result
}

pub fn hash_sha1(input: &str) -> String {
    let data = input.as_bytes();
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    let final_hash = hasher.finalize();

    let result: String = Iterator::map(final_hash.iter(), |byte| format!("{:02X}", byte)).collect();

    tracing::info!("SHA-1 Hashed String {input} to {result}");
    result
}
