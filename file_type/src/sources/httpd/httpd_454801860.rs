use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_454801860: FileType = FileType {
    file_format: &FileFormat {
        id: 454_801_860,
        source_type: SourceType::Httpd,
        name: "fst",
        extensions: &["fst"],
        media_types: &["image/vnd.fst"],
        signatures: &[],
        related_formats: &[],
    },
};
