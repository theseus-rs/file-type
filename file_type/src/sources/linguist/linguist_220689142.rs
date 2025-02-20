use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_220689142: FileType = FileType {
    file_format: &FileFormat {
        id: 220_689_142,
        source_type: SourceType::Linguist,
        name: "Julia REPL",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
