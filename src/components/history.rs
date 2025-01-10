pub struct HistoryItem {
    src: String,
    translated: String
}

impl HistoryItem {
    pub fn new(src: String, translated: String) -> Self {
        HistoryItem {src, translated}
    }

    pub fn src(&self) -> &String {
        &self.src
    }

    pub fn translated(&self) -> &String {
        &self.translated
    }
}