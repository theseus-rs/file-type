use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_332: FileType = FileType {
    file_format: &FileFormat {
        id: 332,
        source_type: SourceType::Linguist,
        name: "SQF",
        extensions: &["hqf", "sqf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
