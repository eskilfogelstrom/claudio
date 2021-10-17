use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Mode {
    Send,
    Return
}

