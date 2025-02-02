use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15897574654348769304: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geospace",
    extensions: &["g3w"],
    media_types: &["application/vnd.geospace"],
    internal_signatures: &[],
    related_formats: &[],
};
