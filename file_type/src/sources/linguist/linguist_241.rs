use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
