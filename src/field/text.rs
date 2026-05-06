#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text {
    pub tag: String,
    pub label: String,
    pub value: String,
}
impl Text {
    pub fn new(tag: String, label: String, value: String) -> Self {
        Self { tag, label, value }
    }
}
