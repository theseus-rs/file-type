use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_58922719: FileType = FileType {
    file_format: &FileFormat {
        id: 58_922_719,
        source_type: SourceType::Httpd,
        name: "silk",
        extensions: &["sil"],
        media_types: &["audio/silk"],
        signatures: &[],
        related_formats: &[],
    },
};
