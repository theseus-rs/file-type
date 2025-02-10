use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1298869663: FileType = FileType {
    file_format: &FileFormat {
        id: 1_298_869_663,
        source_type: SourceType::Iana,
        name: "vnd.think-cell.ppttc+json",
        extensions: &[],
        media_types: &["application/vnd.think-cell.ppttc+json"],
        signatures: &[],
        related_formats: &[],
    },
};
