use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_217: FileType = FileType {
    file_format: &FileFormat {
        id: 217,
        source_type: SourceType::Linguist,
        name: "MAXScript",
        extensions: &["mcr", "ms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
