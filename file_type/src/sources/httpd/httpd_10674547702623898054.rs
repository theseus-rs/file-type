use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10674547702623898054: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dssc xml",
    extensions: &["xdssc"],
    media_types: &["application/dssc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
