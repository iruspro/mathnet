#[derive(Debug,Clone)]
pub enum Msg {
    NavigateTo(String),
    UrlChanged,
    Unchanged,
}