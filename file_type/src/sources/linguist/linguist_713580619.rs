use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_713580619: FileType = FileType {
    file_format: &FileFormat {
        id: 713_580_619,
        source_type: SourceType::Linguist,
        name: "Wren",
        extensions: &["wren"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
