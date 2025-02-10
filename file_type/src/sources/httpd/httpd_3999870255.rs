use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3999870255: FileType = FileType {
    file_format: &FileFormat {
        id: 3_999_870_255,
        source_type: SourceType::Httpd,
        name: "mophun application",
        extensions: &["mpn"],
        media_types: &["application/vnd.mophun.application"],
        signatures: &[],
        related_formats: &[],
    },
};
