use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_886513491: FileFormat = FileFormat {
    id: 886_513_491,
    source_type: SourceType::Httpd,
    name: "emma xml",
    extensions: &["emma"],
    media_types: &["application/emma+xml"],
    signatures: &[],
    related_formats: &[],
};
