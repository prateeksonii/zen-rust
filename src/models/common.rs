use serde::Serialize;

#[derive(Serialize)]
pub struct ResBody<T> {
    pub ok: bool,
    pub result: T,
}
