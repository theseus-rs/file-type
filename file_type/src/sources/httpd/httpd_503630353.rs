use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_503630353: FileType = FileType {
    file_format: &FileFormat {
        id: 503_630_353,
        source_type: SourceType::Httpd,
        name: "mp21",
        extensions: &["m21", "mp21"],
        media_types: &["application/mp21"],
        signatures: &[],
        related_formats: &[],
    },
};
