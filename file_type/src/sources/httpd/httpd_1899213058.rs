use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1899213058: FileType = FileType {
    file_format: &FileFormat {
        id: 1_899_213_058,
        source_type: SourceType::Httpd,
        name: "mobius daf",
        extensions: &["daf"],
        media_types: &["application/vnd.mobius.daf"],
        signatures: &[],
        related_formats: &[],
    },
};
