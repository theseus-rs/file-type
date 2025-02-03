use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_992198684389833801: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkixcmp",
    extensions: &["pki"],
    media_types: &["application/pkixcmp"],
    internal_signatures: &[],
    related_formats: &[],
};
