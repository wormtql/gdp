pub trait FileSystem {
    fn exists(&self, path: &str) -> bool;

    fn read(&self, path: &str) -> Option<String>;

    fn read_serde(&self, path: &str) -> Option<serde_json::Value>;
}
