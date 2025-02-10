use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2170: FileType = FileType {
    file_format: &FileFormat {
        id: 2_170,
        source_type: SourceType::Pronom,
        name: "FamilyTree Maker Database",
        extensions: &["ftw", "fbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
