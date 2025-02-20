use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_99: FileType = FileType {
    file_format: &FileFormat {
        id: 99,
        source_type: SourceType::Linguist,
        name: "Eiffel",
        extensions: &["e"],
        media_types: &["text/x-eiffel"],
        signatures: &[],
        related_formats: &[],
    },
};
