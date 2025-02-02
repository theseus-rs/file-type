use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13429870797822209580: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "audiograph",
    extensions: &["aep"],
    media_types: &["application/vnd.audiograph"],
    internal_signatures: &[],
    related_formats: &[],
};
