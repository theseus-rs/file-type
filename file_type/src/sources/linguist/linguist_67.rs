use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_67: FileType = FileType {
    file_format: &FileFormat {
        id: 67,
        source_type: SourceType::Linguist,
        name: "Component Pascal",
        extensions: &["cp", "cps"],
        media_types: &["text/x-pascal"],
        signatures: &[],
        related_formats: &[],
    },
};
