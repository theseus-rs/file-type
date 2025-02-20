use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1799191154: FileType = FileType {
    file_format: &FileFormat {
        id: 1_799_191_154,
        source_type: SourceType::Httpd,
        name: "oda",
        extensions: &["oda"],
        media_types: &["application/oda"],
        signatures: &[],
        related_formats: &[],
    },
};
