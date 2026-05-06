#[derive(Default, Clone, Debug, PartialEq)]
pub struct Number {
    pub tag: String,
    pub label: String,
    pub value: f64,
}
impl Number {
    pub fn new(tag: String, label: String, value: f64) -> Self {
        Self { tag, label, value }
    }
}
