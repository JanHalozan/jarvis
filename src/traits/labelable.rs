pub trait Labelable {
    fn from_label(label: &str) -> Self;
    fn labels() -> Vec<String>;
}