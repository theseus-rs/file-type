use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2685217944: FileType = FileType {
    file_format: &FileFormat {
        id: 2_685_217_944,
        source_type: SourceType::Iana,
        name: "vnd.palm",
        extensions: &[],
        media_types: &["application/vnd.palm"],
        signatures: &[],
        related_formats: &[],
    },
};
