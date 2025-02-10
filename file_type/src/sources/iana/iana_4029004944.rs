use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4029004944: FileType = FileType {
    file_format: &FileFormat {
        id: 4_029_004_944,
        source_type: SourceType::Iana,
        name: "xml",
        extensions: &[],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
