use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16541584359049439863: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "jpeg",
    extensions: &["jpeg", "jpg", "jpe"],
    media_types: &["image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
