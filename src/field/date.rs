#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Date {
    pub tag: String,
    pub label: String,
    pub value: String,
}
impl Date {
    pub fn new(tag: String, label: String, value: String) -> Self {
        Self { tag, label, value }
    }
}
