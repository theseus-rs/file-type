use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2171: FileType = FileType {
    file_format: &FileFormat {
        id: 2_171,
        source_type: SourceType::Pronom,
        name: "FamilyTree Maker Database",
        extensions: &["ftw", "fbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
