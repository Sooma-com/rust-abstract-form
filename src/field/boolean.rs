#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Boolean {
    pub tag: String,
    pub label: String,
    pub value: bool,
}
impl Boolean {
    pub fn new(tag: String, label: String, value: bool) -> Self {
        Self { tag, label, value }
    }
}
