use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_241: FileType = FileType {
    file_format: &FileFormat {
        id: 241,
        source_type: SourceType::Linguist,
        name: "NL",
        extensions: &["nl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
