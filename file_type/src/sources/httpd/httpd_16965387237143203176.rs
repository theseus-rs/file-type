use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16965387237143203176: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mets xml",
    extensions: &["mets"],
    media_types: &["application/mets+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
