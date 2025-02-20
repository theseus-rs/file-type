use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_350: FileType = FileType {
    file_format: &FileFormat {
        id: 350,
        source_type: SourceType::Linguist,
        name: "Slim",
        extensions: &["slim"],
        media_types: &["text/x-slim"],
        signatures: &[],
        related_formats: &[],
    },
};
