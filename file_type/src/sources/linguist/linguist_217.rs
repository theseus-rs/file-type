use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
