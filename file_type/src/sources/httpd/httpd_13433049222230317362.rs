use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13433049222230317362: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkix cert",
    extensions: &["cer"],
    media_types: &["application/pkix-cert"],
    internal_signatures: &[],
    related_formats: &[],
};
