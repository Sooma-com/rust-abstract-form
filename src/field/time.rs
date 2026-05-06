#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Time {
    pub tag: String,
    pub label: String,
    pub value: String,
}
impl Time {
    pub fn new(tag: String, label: String, value: String) -> Self {
        Self { tag, label, value }
    }
}
