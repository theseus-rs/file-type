use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1396309803: FileType = FileType {
    file_format: &FileFormat {
        id: 1_396_309_803,
        source_type: SourceType::Httpd,
        name: "hhe lesson player",
        extensions: &["les"],
        media_types: &["application/vnd.hhe.lesson-player"],
        signatures: &[],
        related_formats: &[],
    },
};
