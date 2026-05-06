#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub tag: String,
    pub label: String,
    pub value: String,
}
impl DateTime {
    pub fn new(tag: String, label: String, value: String) -> Self {
        Self { tag, label, value }
    }
}
