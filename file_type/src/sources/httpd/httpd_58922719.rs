use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_58922719: FileFormat = FileFormat {
    id: 58_922_719,
    source_type: SourceType::Httpd,
    name: "silk",
    extensions: &["sil"],
    media_types: &["audio/silk"],
    signatures: &[],
    related_formats: &[],
};
