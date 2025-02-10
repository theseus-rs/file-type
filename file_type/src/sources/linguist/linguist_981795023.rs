use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_981795023: FileType = FileType {
    file_format: &FileFormat {
        id: 981_795_023,
        source_type: SourceType::Linguist,
        name: "TextMate Properties",
        extensions: &[],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
