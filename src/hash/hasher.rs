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
