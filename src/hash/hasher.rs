pub fn hash_md5(input: &str) -> String {
    let data = input.as_bytes();
    let digest = md5::compute(data);
    let result = format!("{digest:x}");
    tracing::info!("Hashed String {input} to {result}");
    result
}
