use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17607384768001508299: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "bcpio",
    extensions: &["bcpio"],
    media_types: &["application/x-bcpio"],
    internal_signatures: &[],
    related_formats: &[],
};
