use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4399502372078247101: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wasm",
    extensions: &["wasm"],
    media_types: &["application/wasm"],
    internal_signatures: &[],
    related_formats: &[],
};
