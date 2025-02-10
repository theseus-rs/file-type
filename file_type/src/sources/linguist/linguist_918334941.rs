use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_918334941: FileType = FileType {
    file_format: &FileFormat {
        id: 918_334_941,
        source_type: SourceType::Linguist,
        name: "TSQL",
        extensions: &["sql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
