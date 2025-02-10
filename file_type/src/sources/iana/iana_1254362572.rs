use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1254362572: FileType = FileType {
    file_format: &FileFormat {
        id: 1_254_362_572,
        source_type: SourceType::Iana,
        name: "vnd.dtg.local.html",
        extensions: &[],
        media_types: &["application/vnd.dtg.local.html"],
        signatures: &[],
        related_formats: &[],
    },
};
