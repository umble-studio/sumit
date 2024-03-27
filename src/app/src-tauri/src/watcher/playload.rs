#[derive(Clone, serde::Serialize)]
pub struct ChangedPayload {
    pub(super) path: String,
    pub(super) is_dir: bool,
}

#[derive(Clone, serde::Serialize)]
pub struct RenamedPayload {
    pub(super) path: String,
    pub(super) is_dir: bool,
}

#[derive(Clone, serde::Serialize)]
pub struct AssemblyUpdatedPayload {
    pub(super) path: String,
    pub(super) is_dir: bool,
    pub(super) buffer: &'static [u8],
}
