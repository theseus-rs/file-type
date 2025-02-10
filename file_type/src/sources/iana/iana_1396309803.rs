use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1396309803: FileType = FileType {
    file_format: &FileFormat {
        id: 1_396_309_803,
        source_type: SourceType::Iana,
        name: "vnd.hhe.lesson-player",
        extensions: &[],
        media_types: &["application/vnd.hhe.lesson-player"],
        signatures: &[],
        related_formats: &[],
    },
};
