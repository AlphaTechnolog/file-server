pub fn get_base_path() -> String {
    std::env::var("STORAGE_PATH").expect("STORAGE_PATH must be set")
}
